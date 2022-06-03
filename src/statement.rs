use crate::expression::Expression;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Statement {
    LocalSet(String, Expression),
    Return(Expression),
    Unreachable,
    Nop,
    GlobalSet(String, Expression),
    Unassigned(Expression),
    I32Store(Expression, Expression, u8, u64),
    I64Store(Expression, Expression, u8, u64),
    F32Store(Expression, Expression, u8, u64),
    F64Store(Expression, Expression, u8, u64),
    I32Store8(Expression, Expression, u8, u64),
    I32Store16(Expression, Expression, u8, u64),
    I64Store8(Expression, Expression, u8, u64),
    I64Store16(Expression, Expression, u8, u64),
    I64Store32(Expression, Expression, u8, u64),
    Drop(Expression),
}

impl Statement {
    pub fn emit_code(&self) -> String {
        match self {
            Self::LocalSet(index, expr) => format!("{} = {};", index, expr.emit_code()),
            Self::Return(expr) => format!("return {};", expr.emit_code()),
            Self::Unreachable => "unreachable!();".to_string(),
            Self::Nop => ";".to_string(),
            Self::GlobalSet(index, expr) => format!("{} = {};", index, expr.emit_code()),
            Self::Unassigned(expr) => expr.emit_code(),
            Self::I32Store(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 2 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i32).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i32).cast::<u8>().add({offset}).cast::<i32>().{method}({value_code});")
                }
            },
            Self::I64Store(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 3 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i64).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i64).cast::<u8>().add({offset}).cast::<i64>().{method}({value_code});")
                }
            },
            Self::F32Store(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 2 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut f32).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut f32).cast::<u8>().add({offset}).cast::<f32>().{method}({value_code});")
                }
            },
            Self::F64Store(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 3 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut f64).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut f64).cast::<u8>().add({offset}).cast::<f64>().{method}({value_code});")
                }
            },
            Self::I32Store8(ptr_expr, value_expr, _, offset) => {
                let method = "write";
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i8).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i8).add({offset}).{method}({value_code});")
                }
            },
            Self::I32Store16(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 1 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i16).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i16).cast::<u8>().add({offset}).cast::<i16>().{method}({value_code});")
                }
            },
            Self::I64Store8(ptr_expr, value_expr, _, offset) => {
                let method = "write";
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i8).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i8).add({offset}).{method}({value_code});")
                }
            },
            Self::I64Store16(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 1 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i16).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i16).cast::<u8>().add({offset}).cast::<i16>().{method}({value_code});")
                }
            },
            Self::I64Store32(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 2 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    format!("({ptr_code} as *mut i32).{method}({value_code});")
                } else {
                    format!("({ptr_code} as *mut i32).cast::<u8>().add({offset}).cast::<i32>().{method}({value_code});")
                }
            },
            Self::Drop(expr) => format!(
                "drop({});",
                expr.emit_code(),
            )
        }
    }
}
