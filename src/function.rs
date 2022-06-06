use itertools::Itertools;
use crate::statement::Statement;
use crate::wasm_type::WASMType;
use crate::func_type::FuncType;
use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug, Clone)]
pub struct Function {
    pub index: u32,
    pub ty: FuncType,
    pub locals: Vec<(u32, WASMType)>,
    pub type_index: u32,
    pub statements: Vec<Statement>,
    pub exported: bool,
    pub export_name: Option<String>,
    pub debug_name: Option<String>,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.exported { 
            write!(f, "{}", self.export_name.as_ref().expect("Export name missing for exported function."))
        } else { 
            write!(f, "__w2r_f{}", self.index)
        }
    }
}

impl Function {
    pub fn emit_code(&self, indentation: usize) -> Vec<String> {
        let mut lines = vec![];

        lines.extend(self.emit_help_text());

        lines.extend(self.emit_signature());

        lines.extend(self.emit_body(indentation));

        lines.push("}".to_string());

        lines
    }

    fn emit_help_text(&self) -> Vec<String> {
        if let Some(ref dnm) = self.debug_name {
            vec![format!("/// Debug name: {}", dnm)]
        } else {
            vec!["".to_string()]
        }
    }

    fn emit_signature(&self) -> Vec<String> {
        let mut lines = Vec::new();

        let return_sig = match self.ty.returns.len() {
            0 => "".to_string(),
            1 => {
                format!(" -> {}", &self.ty.returns[0])
            },
            _ => unimplemented!()
        };

        if self.exported {
            lines.push("#[no_mangle]".to_string());
        }

        lines.push(format!(
            "unsafe fn {}({}){} {{",
            self,
            self.emit_param_types(),
            return_sig
        ));

        lines
    }

    fn emit_body(&self, indentation: usize) -> Vec<String> {
        let mut lines = vec![];
        
        if self.locals.len() > 0 {
            lines.push(self.emit_locals(indentation));
        }

        lines.extend(self.emit_statements(indentation));

        lines
    }

    fn emit_param_types(&self) -> String {
        self.ty.params.iter().enumerate().map(|(i, param)| {
            format!("mut p{}: {}", i, param)
        }).join(", ")
    }

    fn emit_locals(&self, indentation: usize) -> String {
        let mut code = format!("{:indentation$}let (", " ");
        let param_len = self.ty.params.len();

        code.push_str(
            &self.locals.iter().map(|(count, _)| {
                (0..*count).into_iter()
            })
            .flatten()
            .enumerate()
            .map(|(i, _)| format!("mut p{}", i + param_len))
            .join(", ")
        );

        code.push_str("): (");

        code.push_str(
            &self.locals.iter().map(|(count, ty)| {
                (0..*count).map(|_| {
                    format!("{}", ty)
                }).join(", ")
            }).join(", ")
        );

        code.push_str(");");

        code
    }

    fn emit_statements(&self, indentation: usize) -> Vec<String> {
        self.statements.iter().map(|stmt| {
            stmt.emit_code(indentation)
        }).flatten().collect()
    }
}