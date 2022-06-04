use itertools::Itertools;
use std::collections::HashMap;
use crate::function::Function;
use crate::func_type::FuncType;
use crate::expression;
use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug, Clone)]
pub struct Context {
    pub functions: HashMap<u32, FunctionKind>,
    pub types: HashMap<u32, FuncType>
}

#[derive(Default, Clone)]
pub struct ContextBuilder<'cb_lt> {
    types: Option<wasmparser::TypeSectionReader<'cb_lt>>,
    imports: Option<wasmparser::ImportSectionReader<'cb_lt>>,
    funcs: Option<wasmparser::FunctionSectionReader<'cb_lt>>,
    code_sections: Vec<wasmparser::FunctionBody<'cb_lt>>,
    exports: Option<wasmparser::ExportSectionReader<'cb_lt>>,
}

impl Context {
    pub fn get_function_by_index(&self, index: u32) -> Option<&FunctionKind> {
        self.functions.get(&index)
    }

    pub fn builder<'a>() -> ContextBuilder<'a> {
        ContextBuilder::new()
    }

    pub fn emit_code(&self) -> String {
        let mut code = String::new();

        // emit no_main; this may change once the start section is supported
        code.push_str("#![no_main]\n\n");

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
                code.push_str(&format!("#[link(wasm_import_module=\"{module}\")]\nextern {{\n"));
                code.push_str(&functions.iter().map(|func| func.emit_code()).join("\n"));
                code.push_str("}\n\n");
            });

        // emit functions
        self.functions
            .iter()
            .filter(|(_, fk)| matches!(fk, FunctionKind::Defined(_)))
            .for_each(|(_, fk)| {
                if let FunctionKind::Defined(func) = fk {
                    code.push_str(&func.emit_code());
                }
            });

        code
    }
}

impl<'a> ContextBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Context {
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
                                    type_index: ty,
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
                                type_index: func_type_index,
                                index: func_index,
                                ty: ft.clone(),
                                locals: locals.into_iter().map(|(_, t)| t.into()).collect(),
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

        Context {
            functions,
            types,
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
}

#[derive(Debug, Clone)]
pub struct ImportedFunction {
    pub index: u32,
    pub ty: FuncType,
    pub type_index: u32,
    pub module: String,
    pub name: String,
}

impl ImportedFunction {
    pub fn emit_code(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.emit_signature());

        code
    }

    fn emit_signature(&self) -> String {
        let return_sig = match self.ty.returns.len() {
            0 => "".to_string(),
            1 => {
                format!("-> {}", &self.ty.returns[0])
            },
            _ => unimplemented!()
        };

        format!(
            "    #[link_name=\"{}\"]\n    fn {}({}) {};\n",
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