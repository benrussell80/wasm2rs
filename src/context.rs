use itertools::Itertools;
use std::collections::HashMap;
use crate::function::Function;
use crate::func_type::FuncType;
use crate::expression::{self, Expression};
use crate::statement::{INDENTATION, Statement};
use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug, Clone)]
pub struct Context {
    pub functions: HashMap<u32, FunctionKind>,
    pub types: HashMap<u32, FuncType>,
    pub memory_size: usize,
    pub data: Vec<(i32, Vec<u8>)>
}

#[derive(Default, Clone)]
pub struct ContextBuilder<'cb_lt> {
    types: Option<wasmparser::TypeSectionReader<'cb_lt>>,
    imports: Option<wasmparser::ImportSectionReader<'cb_lt>>,
    funcs: Option<wasmparser::FunctionSectionReader<'cb_lt>>,
    code_sections: Vec<wasmparser::FunctionBody<'cb_lt>>,
    exports: Option<wasmparser::ExportSectionReader<'cb_lt>>,
    memory: Option<wasmparser::MemorySectionReader<'cb_lt>>,
    data: Vec<wasmparser::DataSectionReader<'cb_lt>>,
}

impl Context {
    pub fn get_function_by_index(&self, index: u32) -> Option<&FunctionKind> {
        self.functions.get(&index)
    }

    pub fn builder<'a>() -> ContextBuilder<'a> {
        ContextBuilder::new()
    }

    pub fn emit_code(&self) -> Vec<String> {
        let mut lines = Vec::new();

        // emit no_main; this may change once the start section is supported
        lines.push("#![no_main]".to_string());

        // emit "setup" function
        lines.extend(self.emit_setup_function());

        // group imports by module
        self.functions
            .iter()
            .filter(|(_, fk)| matches!(fk, FunctionKind::Imported(_)))
            .map(|(_, fk)| if let FunctionKind::Imported(func) = fk { func } else { unreachable!() })
            .fold(HashMap::new(), |mut map, func| {
                map.entry(func.module.clone()).or_insert_with(|| Vec::new()).push(func.clone());
                map
            })
            .iter()
            .for_each(|(module, functions)| {
                lines.push(format!("#[link(wasm_import_module=\"{module}\")]"));
                lines.push("extern {".to_string());
                for func in functions {
                    lines.extend(func.emit_code())
                }
                // code.push(functions.iter().map(|func| func.emit_code()).join("\n"));
                lines.push("}".to_string());
            });

        // emit functions
        self.functions
            .iter()
            .filter(|(_, fk)| matches!(fk, FunctionKind::Defined(_)))
            .for_each(|(_, fk)| {
                if let FunctionKind::Defined(func) = fk {
                    lines.extend(func.emit_code(INDENTATION));
                }
            });

        lines
    }

    fn emit_setup_function(&self) -> Vec<String> {
        use Statement::*;
        use Expression::*;

        // how many pages do we need to start?
        let mut statements = vec![if let Some(furthest_index) = self.data.iter().map(|(index, bytes)| *index as usize + bytes.len()).max() {
            let extra_pages_needed = (furthest_index / (1 << 16)) + 1;
            Drop(MemoryGrow(Box::new(
                I32Sub(
                    Box::new(I32Const(extra_pages_needed as _)),
                    Box::new(MemorySize),
                )
            )))
        } else {
            Statement::Nop
        }];

        for (i, (index, bytes)) in self.data.iter().enumerate() {
            let mut lines = Vec::new();

            let byte_len = bytes.len();

            lines.push(format!("let g{i} = ::std::ptr::slice_from_raw_parts_mut({index} as *mut u8, {byte_len});"));
            lines.push(format!("for (i, byte) in {bytes:?}.iter().enumerate() {{"));
            lines.push(format!("{:INDENTATION$}(*g{i})[i] = *byte;", " "));
            lines.push("}".to_string());

            statements.push(RawRust(lines));
        }

        let f = Function {
            index: u32::MAX,
            ty: FuncType {
                params: vec![],
                returns: vec![],
            },
            locals: vec![],
            statements,
            exported: true,
            export_name: Some("setup".to_string()),
            debug_name: None,
        };

        f.emit_code(INDENTATION)
    }
}

impl<'a> ContextBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(mut self) -> Context {
        let mut ty_index = 0;
        let mut types: HashMap<u32, FuncType> = HashMap::new();
        let mut func_index = 0;
        let mut functions = HashMap::new();

        // types
        if let Some(mut tys) = self.types {
            loop {
                if let Ok(wasmparser::TypeDef::Func(td)) = tys.read() {
                    types.insert(ty_index, td.into());
                } else {
                    break
                }
                ty_index += 1;
            }
        }
        
        // imports 
        if let Some(mut imports) = self.imports.clone() {
            loop {
                match imports.read() {
                    Ok(wasmparser::Import {
                        ty: wasmparser::TypeRef::Func(ty),
                        module,
                        name
                    }) => {
                        if let Some(ft) = types.get(&ty) {
                            functions.insert(
                                func_index,
                                FunctionKind::Imported(ImportedFunction {
                                    index: func_index,
                                    name: name.to_string(),
                                    module: module.to_string(),
                                    ty: ft.clone()
                                })
                            );
                        }
                        func_index += 1;
                    },
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        }

        // functions
        if let Some(mut funcs) = self.funcs {
            let mut code_iter = self.code_sections.iter();
            loop {
                if let Ok(func_type_index) = funcs.read() {
                    if let Some(code) = code_iter.next() {
                        if let Some(ft) = types.get(&func_type_index) {
                            let locals = code.get_locals_reader().expect("Could not get locals reader").into_iter().collect::<wasmparser::Result<Vec<(u32, wasmparser::Type)>>>().expect("locals");
        
                            let func = Function {
                                index: func_index,
                                ty: ft.clone(),
                                locals: locals.into_iter().map(|(count, t)| (count, t.into())).collect(),
                                statements: vec![],
                                exported: false,
                                export_name: None,
                                debug_name: None,
                            };

                            functions.insert(func_index, FunctionKind::Defined(func));
                            func_index += 1;
                        }
                    }
                } else {
                    break
                }
            }
        }

        // exports
        if let Some(mut exps) = self.exports {
            loop {
                match exps.read() {
                    Ok(wasmparser::Export { name, kind: wasmparser::ExternalKind::Func, index }) => {
                        if let Some(FunctionKind::Defined(f)) = functions.get_mut(&index) {
                            f.exported = true;
                            f.export_name = Some(name.to_string());
                        }
                    },
                    Ok(_) => continue,
                    Err(_) => break
                }
            }
        }

        // convert operators to statements
        let funcs_copy = functions.clone();

        for (index, fk) in functions.iter_mut() {
            if let FunctionKind::Defined(func) = fk {
                let num_imports = if let Some(imps) = self.imports.clone() {
                    imps.get_count()
                } else {
                    0
                };

                if let Some(code) = self.code_sections.get((index - num_imports) as usize) {
                    let operators: Vec<wasmparser::Operator<'a>> = code.get_operators_reader().expect("Could not get ops reader").into_iter().collect::<wasmparser::Result<Vec<wasmparser::Operator>>>().expect("ops");
                    let mut iter = operators.into_iter();
                    func.statements = expression::statements_from_operators(
                        &mut iter,
                        &funcs_copy,
                        None
                    ).unwrap();
                }
            }
        }

        // memory; add exported "setup" function to context
        let memory_size;  // this seems to be the default amount for Rust programs

        if let Some(mut memory) = self.memory {
            memory_size = memory.read().expect("memory type").initial as usize;
        } else {
            memory_size = 16;
        }

        // data sections
        let mut data = Vec::new();

        for datum in self.data.iter_mut() {
            if let Ok(wasmparser::Data { kind: wasmparser::DataKind::Active { memory_index: 0, init_expr }, data: d, .. }) = datum.read() {
                let mut opreader = init_expr.get_operators_reader();
                let index = opreader.read();
                let end = opreader.read();
                if let (Ok(wasmparser::Operator::I32Const { value }), Ok(wasmparser::Operator::End)) = (index, end) {
                    data.push((value, d.into()))
                }
            }
        }

        Context {
            functions,
            types,
            memory_size,
            data
        }
    }

    pub fn set_types(mut self, types: wasmparser::TypeSectionReader<'a>) -> Self {
        self.types = Some(types);
        self
    }
    
    pub fn set_imports(mut self, imports: wasmparser::ImportSectionReader<'a>) -> Self {
        self.imports = Some(imports);
        self
    }
    
    pub fn set_funcs(mut self, funcs: wasmparser::FunctionSectionReader<'a>) -> Self {
        self.funcs = Some(funcs);
        self
    }
    
    pub fn add_code_section(mut self, code_section: wasmparser::FunctionBody<'a>) -> Self {
        self.code_sections.push(code_section);
        self
    }
    
    pub fn set_exports(mut self, exports: wasmparser::ExportSectionReader<'a>) -> Self {
        self.exports = Some(exports);
        self
    }

    pub fn set_memory(mut self, memory: wasmparser::MemorySectionReader<'a>) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn add_data_section(mut self, data_section: wasmparser::DataSectionReader<'a>) -> Self {
        self.data.push(data_section);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ImportedFunction {
    pub index: u32,
    pub ty: FuncType,
    pub module: String,
    pub name: String,
}

impl ImportedFunction {
    pub fn emit_code(&self) -> Vec<String> {
        let mut code = Vec::new();

        code.push(self.emit_signature(INDENTATION));

        code
    }

    fn emit_signature(&self, indentation: usize) -> String {
        let return_sig = match self.ty.returns.len() {
            0 => "".to_string(),
            1 => {
                format!("-> {}", &self.ty.returns[0])
            },
            _ => unimplemented!()
        };

        format!(
            "{:>indentation$}#[link_name=\"{}\"]\n    fn {}({}) {};",
            " ",
            self.name,
            self,
            self.emit_param_types(),
            return_sig
        )
    }

    fn emit_param_types(&self) -> String {
        self.ty.params.iter().enumerate().map(|(i, param)| {
            format!("p{}: {}", i, param)
        }).join(", ")
    }
}

impl Display for ImportedFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "__w2r_f{}", self.index)
    }
}

#[derive(Debug, Clone)]
pub enum FunctionKind {
    Imported(ImportedFunction),
    Defined(Function),
    // table function?
}

impl Display for FunctionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Defined(func) => write!(f, "{}", func),
            Self::Imported(func) => write!(f, "{}", func),
        }
    }
}