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
    pub fn emit_code(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.emit_help_text());

        code.push_str(&self.emit_signature());

        code.push_str(&self.emit_body());

        code.push('\n');

        code
    }

    fn emit_help_text(&self) -> String {
        if let Some(ref dnm) = self.debug_name {
            format!("/// Debug name: {}\n", dnm)
        } else {
            "".to_string()
        }
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
            "{}unsafe fn {}({}) {} ",
            if self.exported { "#[no_mangle]\n" } else { "" },
            self,
            self.emit_param_types(),
            return_sig
        )
    }

    fn emit_body(&self) -> String {
        let mut code = String::from("{\n");

        // indentation would be great
        if self.locals.len() > 0 {
            code.push_str(&self.emit_locals());
        }

        code.push_str(&self.emit_statements());

        code.push_str("\n}\n");

        code
    }

    fn emit_param_types(&self) -> String {
        self.ty.params.iter().enumerate().map(|(i, param)| {
            format!("mut p{}: {}", i, param)
        }).join(", ")
    }

    fn emit_locals(&self) -> String {
        let mut code = "    let (".to_string();
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

        // code.push_str(
        //     &self.locals.iter().map(|(count, _)| {
        //         (0..*count).map(|_| {
        //             format!("mut p{}", i + param_len)
        //         }).join(", ")
        //     }).join(", ")
        // );

        code.push_str("): (");

        code.push_str(
            &self.locals.iter().map(|(count, ty)| {
                (0..*count).map(|_| {
                    format!("{}", ty)
                }).join(", ")
            }).join(", ")
        );

        code.push_str(");\n");

        code
    }

    fn emit_statements(&self) -> String {
        self.statements.iter().map(|stmt| {
            let mut code = "    ".to_string();
            code.push_str(&stmt.emit_code());
            code
        }).join("\n")
    }
}