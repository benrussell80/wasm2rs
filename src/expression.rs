use std::collections::HashMap;
use crate::statement::Statement;
use crate::context::FunctionKind;
use wasmparser::{Operator, BlockType};
use itertools::Itertools;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Select(Box<Expression>, Box<Expression>, Box<Expression>),
    LocalGet(String),
    LocalTee(String, Box<Expression>),
    IfElse(Box<Expression>, Box<Expression>),
    Call(FunctionKind, Vec<Expression>),
    CallIndirect(Vec<Expression>),
    I32Load(Box<Expression>, u8, u64),
    I64Load(Box<Expression>, u8, u64),
    F32Load(Box<Expression>, u8, u64),
    F64Load(Box<Expression>, u8, u64),
    I32Load8S(Box<Expression>, u8, u64),
    I32Load8U(Box<Expression>, u8, u64),
    I32Load16S(Box<Expression>, u8, u64),
    I32Load16U(Box<Expression>, u8, u64),
    I64Load8S(Box<Expression>, u8, u64),
    I64Load8U(Box<Expression>, u8, u64),
    I64Load16S(Box<Expression>, u8, u64),
    I64Load16U(Box<Expression>, u8, u64),
    I64Load32S(Box<Expression>, u8, u64),
    I64Load32U(Box<Expression>, u8, u64),
    I32Eqz(Box<Expression>),
    I32Eq(Box<Expression>, Box<Expression>),
    I32Ne(Box<Expression>, Box<Expression>),
    I32LtS(Box<Expression>, Box<Expression>),
    I32LtU(Box<Expression>, Box<Expression>),
    I32GtS(Box<Expression>, Box<Expression>),
    I32GtU(Box<Expression>, Box<Expression>),
    I32LeS(Box<Expression>, Box<Expression>),
    I32LeU(Box<Expression>, Box<Expression>),
    I32GeS(Box<Expression>, Box<Expression>),
    I32GeU(Box<Expression>, Box<Expression>),
    I64Eqz(Box<Expression>),
    I64Eq(Box<Expression>, Box<Expression>),
    I64Ne(Box<Expression>, Box<Expression>),
    I64LtS(Box<Expression>, Box<Expression>),
    I64LtU(Box<Expression>, Box<Expression>),
    I64GtS(Box<Expression>, Box<Expression>),
    I64GtU(Box<Expression>, Box<Expression>),
    I64LeS(Box<Expression>, Box<Expression>),
    I64LeU(Box<Expression>, Box<Expression>),
    I64GeS(Box<Expression>, Box<Expression>),
    I64GeU(Box<Expression>, Box<Expression>),
    F32Eq(Box<Expression>, Box<Expression>),
    F32Ne(Box<Expression>, Box<Expression>),
    F32Lt(Box<Expression>, Box<Expression>),
    F32Gt(Box<Expression>, Box<Expression>),
    F32Le(Box<Expression>, Box<Expression>),
    F32Ge(Box<Expression>, Box<Expression>),
    F64Eq(Box<Expression>, Box<Expression>),
    F64Ne(Box<Expression>, Box<Expression>),
    F64Lt(Box<Expression>, Box<Expression>),
    F64Gt(Box<Expression>, Box<Expression>),
    F64Le(Box<Expression>, Box<Expression>),
    F64Ge(Box<Expression>, Box<Expression>),
    I32Clz(Box<Expression>),
    I32Ctz(Box<Expression>),
    I32Popcnt(Box<Expression>),
    I32Add(Box<Expression>, Box<Expression>),
    I32Sub(Box<Expression>, Box<Expression>),
    I32Mul(Box<Expression>, Box<Expression>),
    I32DivS(Box<Expression>, Box<Expression>),
    I32DivU(Box<Expression>, Box<Expression>),
    I32RemS(Box<Expression>, Box<Expression>),
    I32RemU(Box<Expression>, Box<Expression>),
    I32And(Box<Expression>, Box<Expression>),
    I32Or(Box<Expression>, Box<Expression>),
    I32Xor(Box<Expression>, Box<Expression>),
    I32Shl(Box<Expression>, Box<Expression>),
    I32ShrS(Box<Expression>, Box<Expression>),
    I32ShrU(Box<Expression>, Box<Expression>),
    I32Rotl(Box<Expression>, Box<Expression>),
    I32Rotr(Box<Expression>, Box<Expression>),
    I64Clz(Box<Expression>),
    I64Ctz(Box<Expression>),
    I64Popcnt(Box<Expression>),
    I64Add(Box<Expression>, Box<Expression>),
    I64Sub(Box<Expression>, Box<Expression>),
    I64Mul(Box<Expression>, Box<Expression>),
    I64DivS(Box<Expression>, Box<Expression>),
    I64DivU(Box<Expression>, Box<Expression>),
    I64RemS(Box<Expression>, Box<Expression>),
    I64RemU(Box<Expression>, Box<Expression>),
    I64And(Box<Expression>, Box<Expression>),
    I64Or(Box<Expression>, Box<Expression>),
    I64Xor(Box<Expression>, Box<Expression>),
    I64Shl(Box<Expression>, Box<Expression>),
    I64ShrS(Box<Expression>, Box<Expression>),
    I64ShrU(Box<Expression>, Box<Expression>),
    I64Rotl(Box<Expression>, Box<Expression>),
    I64Rotr(Box<Expression>, Box<Expression>),
    F32Abs(Box<Expression>),
    F32Neg(Box<Expression>),
    F32Ceil(Box<Expression>),
    F32Floor(Box<Expression>),
    F32Trunc(Box<Expression>),
    F32Nearest(Box<Expression>),
    F32Sqrt(Box<Expression>),
    F32Add(Box<Expression>, Box<Expression>),
    F32Sub(Box<Expression>, Box<Expression>),
    F32Mul(Box<Expression>, Box<Expression>),
    F32Div(Box<Expression>, Box<Expression>),
    F32Min(Box<Expression>, Box<Expression>),
    F32Max(Box<Expression>, Box<Expression>),
    F32Copysign(Box<Expression>, Box<Expression>),
    F64Abs(Box<Expression>),
    F64Neg(Box<Expression>),
    F64Ceil(Box<Expression>),
    F64Floor(Box<Expression>),
    F64Trunc(Box<Expression>),
    F64Nearest(Box<Expression>),
    F64Sqrt(Box<Expression>),
    F64Add(Box<Expression>, Box<Expression>),
    F64Sub(Box<Expression>, Box<Expression>),
    F64Mul(Box<Expression>, Box<Expression>),
    F64Div(Box<Expression>, Box<Expression>),
    F64Min(Box<Expression>, Box<Expression>),
    F64Max(Box<Expression>, Box<Expression>),
    F64Copysign(Box<Expression>, Box<Expression>),
    I32WrapI64(Box<Expression>),
    I32TruncF32S(Box<Expression>),
    I32TruncF32U(Box<Expression>),
    I32TruncF64S(Box<Expression>),
    I32TruncF64U(Box<Expression>),
    I64ExtendI32S(Box<Expression>),
    I64ExtendI32U(Box<Expression>),
    I64TruncF32S(Box<Expression>),
    I64TruncF32U(Box<Expression>),
    I64TruncF64S(Box<Expression>),
    I64TruncF64U(Box<Expression>),
    F32ConvertI32S(Box<Expression>),
    F32ConvertI32U(Box<Expression>),
    F32ConvertI64S(Box<Expression>),
    F32ConvertI64U(Box<Expression>),
    F32DemoteF64(Box<Expression>),
    F64ConvertI32S(Box<Expression>),
    F64ConvertI32U(Box<Expression>),
    F64ConvertI64S(Box<Expression>),
    F64ConvertI64U(Box<Expression>),
    F64PromoteF32(Box<Expression>),
    I32ReinterpretF32(Box<Expression>),
    I64ReinterpretF64(Box<Expression>),
    F32ReinterpretI32(Box<Expression>),
    F64ReinterpretI64(Box<Expression>),
    I32Extend8S(Box<Expression>),
    I32Extend16S(Box<Expression>),
    I64Extend8S(Box<Expression>),
    I64Extend16S(Box<Expression>),
    I64Extend32S(Box<Expression>),
    I32Const(i32),  // or u32?
    I64Const(i64),  // or u64?
    F32Const(u32),
    F64Const(u64),
}

impl<'a> Expression {
    pub fn emit_code(&self) -> String {
        match self {
            Self::Select(expr1, expr2, cond) => format!(
                "{{ let e1 = {}; let e2 = {}; if {} == 1 {{ e1 }} else {{ e2 }} }}",
                expr1.emit_code(),
                expr2.emit_code(),
                cond.emit_code()
            ),
            Self::LocalGet(name) => format!(
                "({})",
                name
            ),
            Self::LocalTee(name, expr) => format!(
                "{{ {} = {}; {} }}",
                name,
                expr.emit_code(),
                name
            ),
            // Self::IfElse(Box<Expression>, Box<Expression>),
            Self::Call(func, args) => format!(
                "({}({}))",
                func,
                args.iter().map(|arg| arg.emit_code()).join(",")
            ),
            // Self::CallIndirect(Vec<Expression>),
            Self::I32Load(expr, align, offset) => {
                let method = if *align == 2 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();
                if *offset == 0 {
                    format!("(({expr_code} as *const i32).{method}())")
                } else {
                    format!("(({expr_code} as *const i32).cast::<u8>().add({offset}).cast::<i32>().{method}())")
                }
            },
            Self::I64Load(expr, align, offset) => {
                let method = if *align == 3 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const i64).{method}())")
                } else {
                    format!("(({expr_code} as *const i64).cast::<u8>().add({offset}).cast::<i64>().{method}())")
                }
            },
            Self::F32Load(expr, align, offset) => {
                let method = if *align == 2 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const f32).{method}())")
                } else {
                    format!("(({expr_code} as *const f32).cast::<u8>().add({offset}).cast::<f32>().{method}())")
                }
            },
            Self::F64Load(expr, align, offset) => {
                let method = if *align == 3 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const f64).{method}())")
                } else {
                    format!("(({expr_code} as *const f64).cast::<u8>().add({offset}).cast::<f64>().{method}())")
                }
            },
            Self::I32Load8S(expr, _, offset) => {
                let expr_code = expr.emit_code();
                let method = "read";

                if *offset == 0 {
                    format!("(({expr_code} as *const i8).{method}() as i32)")
                } else {
                    format!("(({expr_code} as *const i8).add({offset}).{method}() as i32)")
                }
            },
            Self::I32Load8U(expr, _, offset) => {
                let expr_code = expr.emit_code();
                let method = "read";

                if *offset == 0 {
                    format!("(({expr_code} as *const i8).{method}() as i32)")
                } else {
                    format!("(({expr_code} as *const i8).add({offset}).{method}() as i32)")
                }
            },
            Self::I32Load16S(expr, align, offset) => {
                let method = if *align == 1 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const i16).{method}() as i32)")
                } else {
                    format!("(({expr_code} as *const i16).cast::<u8>().add({offset}).cast::<i16>().{method}() as i32)")
                }
            },
            Self::I32Load16U(expr, align, offset) => {
                let method = if *align == 1 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const u16).{method}() as i32)")
                } else {
                    format!("(({expr_code} as *const u16).cast::<u8>().add({offset}).cast::<u16>().{method}() as i32)")
                }
            },
            Self::I64Load8S(expr, _, offset) => {
                let expr_code = expr.emit_code();
                let method = "read";

                if *offset == 0 {
                    format!("(({expr_code} as *const i8).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const i8).add({offset}).{method}() as i64)")
                }
            },
            Self::I64Load8U(expr, _, offset) => {
                let expr_code = expr.emit_code();
                let method = "read";

                if *offset == 0 {
                    format!("(({expr_code} as *const u8).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const u8).add({offset}).{method}() as i64)")
                }
            },
            Self::I64Load16S(expr, align, offset) => {
                let method = if *align == 1 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const i16).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const i16).cast::<u8>().add({offset}).cast::<i16>().{method}() as i64)")
                }
            },
            Self::I64Load16U(expr, align, offset) => {
                let method = if *align == 1 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const u16).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const u16).cast::<u8>().add({offset}).cast::<u16>().{method}() as i64)")
                }
            },
            Self::I64Load32S(expr, align, offset) => {
                let method = if *align == 2 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const i32).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const i32).cast::<u8>().add({offset}).cast::<i32>().{method}() as i64)")
                }
            },
            Self::I64Load32U(expr, align, offset) => {
                let method = if *align == 2 {
                    "read"
                } else {
                    "read_unaligned"
                };
                let expr_code = expr.emit_code();

                if *offset == 0 {
                    format!("(({expr_code} as *const u32).{method}() as i64)")
                } else {
                    format!("(({expr_code} as *const u32).cast::<u8>().add({offset}).cast::<u32>().{method}() as i64)")
                }
            },
            Self::I32Eqz(expr) => format!(
                "(({} == 0i32) as i32)",
                expr.emit_code(),
            ),
            Self::I32Eq(expr1, expr2) => format!(
                "(({} == {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Ne(expr1, expr2) => format!(
                "(({} != {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32LtS(expr1, expr2) => format!(
                "(({} < {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32LtU(expr1, expr2) => format!(
                "((({} as u32) < ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32GtS(expr1, expr2) => format!(
                "(({} > {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32GtU(expr1, expr2) => format!(
                "((({} as u32) > ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32LeS(expr1, expr2) => format!(
                "(({} <= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32LeU(expr1, expr2) => format!(
                "((({} as u32) <= ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32GeS(expr1, expr2) => format!(
                "(({} >= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32GeU(expr1, expr2) => format!(
                "((({} as u32) >= ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Eqz(expr) => format!(
                "(({} == 0i64) as i32)",
                expr.emit_code(),
            ),
            Self::I64Eq(expr1, expr2) => format!(
                "(({} == {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Ne(expr1, expr2) => format!(
                "(({} != {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64LtS(expr1, expr2) => format!(
                "(({} < {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64LtU(expr1, expr2) => format!(
                "((({} as u64) < ({} as u64)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64GtS(expr1, expr2) => format!(
                "(({} > {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64GtU(expr1, expr2) => format!(
                "((({} as u64) > ({} as u64)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64LeS(expr1, expr2) => format!(
                "(({} <= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64LeU(expr1, expr2) => format!(
                "((({} as u64) <= ({} as u64)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64GeS(expr1, expr2) => format!(
                "(({} >= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64GeU(expr1, expr2) => format!(
                "((({} as u64) >= ({} as u64)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Eq(expr1, expr2) => format!(
                "(({} == {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Ne(expr1, expr2) => format!(
                "(({} != {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Lt(expr1, expr2) => format!(
                "(({} < {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Gt(expr1, expr2) => format!(
                "(({} > {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Le(expr1, expr2) => format!(
                "(({} <= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Ge(expr1, expr2) => format!(
                "(({} >= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Eq(expr1, expr2) => format!(
                "(({} == {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Ne(expr1, expr2) => format!(
                "(({} != {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Lt(expr1, expr2) => format!(
                "(({} < {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Gt(expr1, expr2) => format!(
                "(({} > {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Le(expr1, expr2) => format!(
                "(({} <= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Ge(expr1, expr2) => format!(
                "(({} >= {}) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Clz(expr) => format!(
                "({}.leading_zeros())",
                expr.emit_code(),
            ),
            Self::I32Ctz(expr) => format!(
                "({}.trailing_zeros())",
                expr.emit_code(),
            ),
            Self::I32Popcnt(expr) => format!(
                "({}.count_ones())",
                expr.emit_code(),
            ),
            Self::I32Add(expr1, expr2) => format!(
                "({} + {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Sub(expr1, expr2) => format!(
                "({} - {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Mul(expr1, expr2) => format!(
                "({} * {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32DivS(expr1, expr2) => format!(
                "({} / {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32DivU(expr1, expr2) => format!(
                "((({} as u32) / ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32RemS(expr1, expr2) => format!(
                "({} % {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32RemU(expr1, expr2) => format!(
                "((({} as u32) % ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32And(expr1, expr2) => format!(
                "({} & {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Or(expr1, expr2) => format!(
                "({} | {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Xor(expr1, expr2) => format!(
                "({} ^ {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Shl(expr1, expr2) => format!(
                "({} << ({} as u32))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32ShrS(expr1, expr2) => format!(
                "({} >> {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32ShrU(expr1, expr2) => format!(
                "((({} as u32) >> ({} as u32)) as i32)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Rotl(expr1, expr2) => format!(
                "({}.rotate_left({} as u32))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32Rotr(expr1, expr2) => format!(
                "({}.rotate_right({} as u32))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Clz(expr) => format!(
                "({}.leading_zeros() as i64)",
                expr.emit_code(),
            ),
            Self::I64Ctz(expr) => format!(
                "({}.trailing_zeros() as i64)",
                expr.emit_code(),
            ),
            Self::I64Popcnt(expr) => format!(
                "({}.count_ones() as i64)",
                expr.emit_code(),
            ),
            Self::I64Add(expr1, expr2) => format!(
                "({} + {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Sub(expr1, expr2) => format!(
                "({} - {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Mul(expr1, expr2) => format!(
                "({} * {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64DivS(expr1, expr2) => format!(
                "({} / {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64DivU(expr1, expr2) => format!(
                "((({} as u64) / ({} as u64)) as i64)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64RemS(expr1, expr2) => format!(
                "({} % {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64RemU(expr1, expr2) => format!(
                "((({} as u64) % ({} as u64)) as i64)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64And(expr1, expr2) => format!(
                "({} & {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Or(expr1, expr2) => format!(
                "({} | {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Xor(expr1, expr2) => format!(
                "({} ^ {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Shl(expr1, expr2) => format!(
                "({} << ({} as u64))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64ShrS(expr1, expr2) => format!(
                "({} >> {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64ShrU(expr1, expr2) => format!(
                "((({} as u64) >> ({} as u64)) as i64)",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Rotl(expr1, expr2) => format!(
                "({}.rotate_left({} as u32))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I64Rotr(expr1, expr2) => format!(
                "({}.rotate_right({} as u32))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Abs(expr) => format!(
                "({}.abs())",
                expr.emit_code(),
            ),
            Self::F32Neg(expr) => format!(
                "(-{})",
                expr.emit_code(),
            ),
            Self::F32Ceil(expr) => format!(
                "({}.ceil())",
                expr.emit_code(),
            ),
            Self::F32Floor(expr) => format!(
                "({}.floor())",
                expr.emit_code(),
            ),
            Self::F32Trunc(expr) => format!(
                "({}.trunc())",
                expr.emit_code(),
            ),
            Self::F32Nearest(expr) => format!(
                "({}.round())",
                expr.emit_code(),
            ),
            Self::F32Sqrt(expr) => format!(
                "({}.sqrt())",
                expr.emit_code(),
            ),
            Self::F32Add(expr1, expr2) => format!(
                "({} + {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Sub(expr1, expr2) => format!(
                "({} - {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Mul(expr1, expr2) => format!(
                "({} * {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F32Div(expr1, expr2) => format!(
                "({} / {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            // Self::F32Min(expr, expr) => format!(
            //     "",
            //     expr.emit_code(),
            // ),
            // Self::F32Max(expr, expr) => format!(
            //     "",
            //     expr.emit_code(),
            // ),
            Self::F32Copysign(expr1, expr2) => format!(
                "({}.copysign({}))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Abs(expr) => format!(
                "({}.abs())",
                expr.emit_code(),
            ),
            Self::F64Neg(expr) => format!(
                "(-{})",
                expr.emit_code(),
            ),
            Self::F64Ceil(expr) => format!(
                "({}.ceil())",
                expr.emit_code(),
            ),
            Self::F64Floor(expr) => format!(
                "({}.floor())",
                expr.emit_code(),
            ),
            Self::F64Trunc(expr) => format!(
                "({}.trunc())",
                expr.emit_code(),
            ),
            Self::F64Nearest(expr) => format!(
                "({}.round())",
                expr.emit_code(),
            ),
            Self::F64Sqrt(expr) => format!(
                "({}.sqrt())",
                expr.emit_code(),
            ),
            Self::F64Add(expr1, expr2) => format!(
                "({} + {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Sub(expr1, expr2) => format!(
                "({} - {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Mul(expr1, expr2) => format!(
                "({} * {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::F64Div(expr1, expr2) => format!(
                "({} / {})",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            // Self::F64Min(expr, expr) => format!(
            //     "",
            //     expr.emit_code(),
            // ),
            // Self::F64Max(expr, expr) => format!(
            //     "",
            //     expr.emit_code(),
            // ),
            Self::F64Copysign(expr1, expr2) => format!(
                "({}.copysign({}))",
                expr1.emit_code(),
                expr2.emit_code(),
            ),
            Self::I32WrapI64(expr) => format!(
                "({} as i32)",
                expr.emit_code(),
            ),
            Self::I32TruncF32S(expr) => format!(
                "({}.to_int_unchecked::<i32>())",
                expr.emit_code(),
            ),
            Self::I32TruncF32U(expr) => format!(
                "({}.to_int_unchecked::<u32>() as i32)",
                expr.emit_code(),
            ),
            Self::I32TruncF64S(expr) => format!(
                "({}.to_int_unchecked::<i32>())",
                expr.emit_code(),
            ),
            Self::I32TruncF64U(expr) => format!(
                "({}.to_int_unchecked::<u32>() as i32)",
                expr.emit_code(),
            ),
            Self::I64ExtendI32S(expr) => format!(
                "({} as i64)",
                expr.emit_code(),
            ),
            Self::I64ExtendI32U(expr) => format!(
                "(({} as u64) as i64)",
                expr.emit_code(),
            ),
            Self::I64TruncF32S(expr) => format!(
                "({}.to_int_unchecked::<i32>())",
                expr.emit_code(),
            ),
            Self::I64TruncF32U(expr) => format!(
                "({}.to_int_unchecked::<u32>() as i32)",
                expr.emit_code(),
            ),
            Self::I64TruncF64S(expr) => format!(
                "({}.to_int_unchecked::<i32>())",
                expr.emit_code(),
            ),
            Self::I64TruncF64U(expr) => format!(
                "({}.to_int_unchecked::<u32>() as i32)",
                expr.emit_code(),
            ),
            Self::F32ConvertI32S(expr)
            | Self::F32ConvertI32U(expr)
            | Self::F32ConvertI64S(expr)
            | Self::F32ConvertI64U(expr)
            | Self::F32DemoteF64(expr) => format!(
                "({} as f32)",
                expr.emit_code(),
            ),
            Self::F64ConvertI32S(expr)
            | Self::F64ConvertI32U(expr)
            | Self::F64ConvertI64S(expr)
            | Self::F64ConvertI64U(expr)
            | Self::F64PromoteF32(expr) => format!(
                "({} as f64)",
                expr.emit_code(),
            ),
            Self::I32ReinterpretF32(expr) => format!(
                "(::std::mem::transmute::<f32, i32>({}))",
                expr.emit_code(),
            ),
            Self::I64ReinterpretF64(expr) => format!(
                "(::std::mem::transmute::<f64, i64>({}))",
                expr.emit_code(),
            ),
            Self::F32ReinterpretI32(expr) => format!(
                "(::std::mem::transmute::<i32, f32>({}))",
                expr.emit_code(),
            ),
            Self::F64ReinterpretI64(expr) => format!(
                "(::std::mem::transmute::<i64, f64>({}))",
                expr.emit_code(),
            ),
            Self::I32Const(num) => format!(
                "({}i32)",
                num
            ),  // or u32?
            Self::I64Const(num) => format!(
                "({}i64)",
                num
            ),  // or u64?
            Self::F32Const(num) => format!(
                "(f32::from_bits({}))",
                num
            ),
            Self::F64Const(num) => format!(
                "(f64::from_bits({}))",
                num
            ),
            Self::I32Extend8S(_) => unimplemented!(),
            Self::I32Extend16S(_) => unimplemented!(),
            Self::I64Extend8S(_) => unimplemented!(),
            Self::I64Extend16S(_) => unimplemented!(),
            Self::I64Extend32S(_) => unimplemented!(),
            Self::IfElse(_, _) => unimplemented!(),
            Self::CallIndirect(_) => unimplemented!(),
            Self::F32Max(_, _) => unimplemented!(),
            Self::F32Min(_, _) => unimplemented!(),
            Self::F64Max(_, _) => unimplemented!(),
            Self::F64Min(_, _) => unimplemented!(),
        }
    }
}

pub enum ParsingContext {
    Block {
        block_depth: u32
    }
}

fn build_block_statement_empty_type<'a>(iter: &mut impl Iterator<Item=Operator<'a>>, functions: &HashMap<u32, FunctionKind>, block_depth: u32) -> Result<Statement, ParserError<'a>> {
    Ok(Statement::Block(
        statements_from_operators(iter, &functions, Some(ParsingContext::Block { block_depth }))?,
        block_depth
    ))
}

pub fn statements_from_operators<'a>(iter: &mut impl Iterator<Item=Operator<'a>>, functions: &HashMap<u32, FunctionKind>, parsing_context: Option<ParsingContext>) -> Result<Vec<Statement>, ParserError<'a>> {
    let mut exprs: Vec<Expression> = vec![];
    let mut stmts: Vec<Statement> = vec![];

    loop {
        if let Some(op) = iter.next() {
            match op {
                Operator::Unreachable => stmts.push(Statement::Unreachable),
                Operator::Nop => stmts.push(Statement::Nop),
                Operator::Block {
                    ty,
                } => {
                    match ty {
                        BlockType::Empty => {
                            stmts.push(build_block_statement_empty_type(
                                iter,
                                &functions,
                                if let Some(ParsingContext::Block { block_depth }) = parsing_context { block_depth + 1 } else { 0 },
                            )?);
                        },
                        BlockType::Type(_typ) => {
                            return Err(ParserError::Unimplemented(op))
                        },
                        BlockType::FuncType(_index) => {
                            return Err(ParserError::Unimplemented(op))
                        },
                    }
                },
                // Operator::Loop {
                //     ty,
                // } => return Err(ParserError::Unimplemented),
                // Operator::If {
                //     ty,
                // } => return Err(ParserError::Unimplemented),
                // Operator::Else => return Err(ParserError::Unimplemented),
                // Operator::Try {
                //     ty,
                // } => return Err(ParserError::Unimplemented),
                // Operator::Catch {
                //     index,
                // } => return Err(ParserError::Unimplemented),
                // Operator::Throw {
                //     index,
                // } => return Err(ParserError::Unimplemented),
                // Operator::Rethrow {
                //     relative_depth,
                // } => return Err(ParserError::Unimplemented),
                Operator::End => {
                    match parsing_context {
                        Some(ParsingContext::Block { .. }) | None => break,
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::Br {
                    relative_depth,
                } => {
                    if let Some(ParsingContext::Block { block_depth }) = parsing_context {
                        stmts.push(Statement::Br { block_depth, relative_depth })
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::BrIf {
                    relative_depth,
                } => {
                    if let Some(cond) = exprs.pop() {
                        if let Some(ParsingContext::Block { block_depth }) = parsing_context {
                            stmts.push(Statement::BrIf { cond, block_depth, relative_depth })
                        } else {
                            return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                        }
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::BrTable {
                    ref table,
                } => {
                    if let Some(cond) = exprs.pop() {
                        if let Some(ParsingContext::Block { block_depth }) = parsing_context {
                            stmts.push(Statement::BrTable {
                                cond,
                                block_depth,
                                table: table.targets().collect::<Result<_, _>>().unwrap(),
                                default: table.default()
                            })
                        }
                    } else {
                        return Err(ParserError::Invalid { statements: stmts, expressions: exprs, operator: op })
                    }
                },
                Operator::Return => {
                    if let Some(expr) = exprs.pop() {
                        stmts.push(Statement::Return(expr))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::Call {
                    function_index,
                } => {
                    if let Some(func) = functions.get(&function_index) {
                        let num_params = match func {
                            FunctionKind::Defined(f) => f.ty.params.len(),
                            FunctionKind::Imported(f) => f.ty.params.len(),
                        };
                        let mut inputs = Vec::with_capacity(num_params);
                        for _ in 0..num_params {
                            if let Some(expr) = exprs.pop() {
                                inputs.push(expr);
                            } else {
                                return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                            }
                        }
                        inputs.reverse();
                        exprs.push(Expression::Call(func.clone(), inputs))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                // Operator::CallIndirect {
                //     index,
                //     table_index,
                //     table_byte,
                // } => return Err(ParserError::Unimplemented),
                // Operator::ReturnCall {
                //     function_index,
                // } => return Err(ParserError::Unimplemented),
                // Operator::ReturnCallIndirect {
                //     index,
                //     table_index,
                // } => return Err(ParserError::Unimplemented),
                // Operator::Delegate {
                //     relative_depth,
                // } => return Err(ParserError::Unimplemented),
                // Operator::CatchAll => return Err(ParserError::Unimplemented),
                Operator::Drop => {
                    if let Some(expr) = exprs.pop() {
                        stmts.push(Statement::Drop(expr))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::Select => {
                    let cond_opt = exprs.pop();
                    let expr2_opt = exprs.pop();
                    let expr1_opt = exprs.pop();
                    match (expr1_opt, expr2_opt, cond_opt) {
                        (Some(expr1), Some(expr2), Some(cond)) => {
                            exprs.push(Expression::Select(Box::new(expr1), Box::new(expr2), Box::new(cond)))
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                // Operator::TypedSelect {
                //     ty: Type,
                // } => return Err(ParserError::Unimplemented),
                Operator::LocalGet {
                    local_index,
                } => {
                    exprs.push(Expression::LocalGet(format!("p{}", local_index)))
                },
                Operator::LocalSet {
                    local_index,
                } => {
                    if let Some(expr) = exprs.pop() {
                        stmts.push(Statement::LocalSet(format!("p{}", local_index), expr))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::LocalTee {
                    local_index,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::LocalTee(format!("p{}", local_index), Box::new(expr)))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                // Operator::GlobalGet {
                //     global_index,
                // } => {
                //     return Err(ParserError::Unimplemented)
                // },
                // Operator::GlobalSet {
                //     global_index,
                // } => {
                //     return Err(ParserError::Unimplemented)
                // },
                Operator::I32Load {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I32Load(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Load {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::F32Load(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Load {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::F64Load(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Load8S {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I32Load8S(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Load8U {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I32Load8U(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Load16S {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I32Load16S(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Load16U {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I32Load16U(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load8S {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load8S(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load8U {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load8U(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load16S {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load16S(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load16U {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load16U(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load32S {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load32S(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Load32U {
                    memarg,
                } => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(Expression::I64Load32U(Box::new(expr), memarg.align, memarg.offset))
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Store {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I32Store(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Store {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I64Store(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Store {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::F32Store(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Store {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::F64Store(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Store8 {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I32Store8(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Store16 {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I32Store16(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Store8 {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I64Store8(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Store16 {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I64Store16(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Store32 {
                    memarg,
                } => {
                    let value_expr = exprs.pop();
                    let ptr_expr = exprs.pop();
                    match (ptr_expr, value_expr) {
                        (Some(pe), Some(ve)) => {
                            stmts.push(Statement::I64Store32(pe, ve, memarg.align, memarg.offset));
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts. clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                // Operator::MemorySize {
                //     mem,
                //     mem_byte,
                // } => return Err(ParserError::Unimplemented),
                // Operator::MemoryGrow {
                //     mem,
                //     mem_byte,
                // } => return Err(ParserError::Unimplemented),
                Operator::I32Const {
                    value,
                } => {
                    exprs.push(Expression::I32Const(value))
                },
                Operator::I64Const {
                    value,
                } => {
                    exprs.push(Expression::I64Const(value))
                },
                Operator::F32Const {
                    value,
                } => {
                    exprs.push(Expression::F32Const(value.bits()))
                },
                Operator::F64Const {
                    value,
                } => {
                    exprs.push(Expression::F64Const(value.bits()))
                },
                // Operator::RefNull {
                //     ty,
                // } => return Err(ParserError::Unimplemented),
                // Operator::RefIsNull => return Err(ParserError::Unimplemented),
                // Operator::RefFunc {
                //     function_index,
                // } => return Err(ParserError::Unimplemented),
                Operator::I32Eqz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Eqz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Eq => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Eq(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Ne => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Ne(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32LtS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32LtS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32LtU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32LtU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32GtS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32GtS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32GtU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32GtU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32LeS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32LeS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32LeU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32LeU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32GeS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32GeS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32GeU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32GeU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Eqz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Eqz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Eq => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Eq(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Ne => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Ne(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64LtS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64LtS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64LtU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64LtU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64GtS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64GtS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64GtU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64GtU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64LeS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64LeS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64LeU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64LeU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64GeS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64GeS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64GeU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64GeU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Eq => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Eq(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Ne => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Ne(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Lt => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Lt(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Gt => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Gt(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Le => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Le(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Ge => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Ge(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Eq => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Eq(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Ne => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Ne(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Lt => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Lt(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Gt => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Gt(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Le => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Le(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Ge => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Ge(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Clz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Clz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I32Ctz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Ctz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I32Popcnt => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Popcnt(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I32Add => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Add(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Sub => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Sub(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Mul => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Mul(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32DivS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32DivS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32DivU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32DivU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32RemS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32RemS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32RemU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32RemU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32And => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32And(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Or => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Or(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Xor => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Xor(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Shl => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Shl(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32ShrS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32ShrS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32ShrU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32ShrU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Rotl => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Rotl(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Rotr => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I32Rotr(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Clz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Eqz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I64Ctz => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Ctz(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I64Popcnt => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Popcnt(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::I64Add => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Add(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Sub => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Sub(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Mul => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Mul(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64DivS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64DivS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64DivU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64DivU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64RemS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64RemS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64RemU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64RemU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64And => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64And(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Or => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Or(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Xor => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Xor(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Shl => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Shl(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64ShrS => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64ShrS(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64ShrU => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64ShrU(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Rotl => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Rotl(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Rotr => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::I64Rotr(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Abs => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Abs(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::F32Neg => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Neg(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::F32Ceil => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Ceil(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }

                },
                Operator::F32Floor => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Floor(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Trunc => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Trunc(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Nearest => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Nearest(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Sqrt => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32Sqrt(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Add => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Add(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Sub => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Sub(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Mul => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Mul(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Div => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Div(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Min => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Min(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Max => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Max(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32Copysign => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F32Copysign(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Abs => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Abs(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Neg => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Neg(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Ceil => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Ceil(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Floor => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Floor(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Trunc => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Trunc(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Nearest => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Nearest(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Sqrt => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64Sqrt(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Add => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Add(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Sub => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Sub(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Mul => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Mul(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Div => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Div(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Min => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Min(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Max => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Max(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64Copysign => {
                    let right = exprs.pop();
                    let left = exprs.pop();
                    match (left, right) {
                        (Some(le), Some(re)) => {
                            exprs.push(
                                Expression::F64Copysign(
                                    Box::new(le),
                                    Box::new(re),
                                )
                            )
                        },
                        _ => return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32WrapI64 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32WrapI64(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32TruncF32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32TruncF32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32TruncF32U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32TruncF32U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32TruncF64S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32TruncF64S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32TruncF64U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32TruncF64U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64ExtendI32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64ExtendI32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64ExtendI32U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64ExtendI32U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64TruncF32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64TruncF32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64TruncF32U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64TruncF32U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64TruncF64S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64TruncF64S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64TruncF64U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64TruncF64U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32ConvertI32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32ConvertI32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32ConvertI32U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32ConvertI32U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32ConvertI64S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32ConvertI64S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32ConvertI64U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32ConvertI64U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32DemoteF64 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32DemoteF64(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64ConvertI32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64ConvertI32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64ConvertI32U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64ConvertI32U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64ConvertI64S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64ConvertI64S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64ConvertI64U => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64ConvertI64U(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64PromoteF32 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64PromoteF32(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32ReinterpretF32 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32ReinterpretF32(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64ReinterpretF64 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64ReinterpretF64(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F32ReinterpretI32 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F32ReinterpretI32(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::F64ReinterpretI64 => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::F64ReinterpretI64(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Extend8S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Extend8S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I32Extend16S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I32Extend16S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Extend8S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Extend8S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Extend16S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Extend16S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
                Operator::I64Extend32S => {
                    if let Some(expr) = exprs.pop() {
                        exprs.push(
                            Expression::I64Extend32S(Box::new(expr))
                        )
                    } else {
                        return Err(ParserError::Invalid { statements: stmts.clone(), expressions: exprs.clone(), operator: op })
                    }
                },
            
                // 0xFC operators
                // Non-trapping Float-to-int Conversions
                // Operator::I32TruncSatF32S => return Err(ParserError::Unimplemented),
                // Operator::I32TruncSatF32U => return Err(ParserError::Unimplemented),
                // Operator::I32TruncSatF64S => return Err(ParserError::Unimplemented),
                // Operator::I32TruncSatF64U => return Err(ParserError::Unimplemented),
                // Operator::I64TruncSatF32S => return Err(ParserError::Unimplemented),
                // Operator::I64TruncSatF32U => return Err(ParserError::Unimplemented),
                // Operator::I64TruncSatF64S => return Err(ParserError::Unimplemented),
                // Operator::I64TruncSatF64U => return Err(ParserError::Unimplemented),
            
                // 0xFC operators
                // bulk memory https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md
                // Operator::MemoryInit {
                //     segment,
                //     mem,
                // } => return Err(ParserError::Unimplemented),
                // Operator::DataDrop {
                //     segment,
                // } => return Err(ParserError::Unimplemented),
                // Operator::MemoryCopy {
                //     src,
                //     dst,
                // } => return Err(ParserError::Unimplemented),
                // Operator::MemoryFill {
                //     mem,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableInit {
                //     segment,
                //     table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::ElemDrop {
                //     segment,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableCopy {
                //     dst_table,
                //     src_table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableFill {
                //     table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableGet {
                //     table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableSet {
                //     table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableGrow {
                //     table,
                // } => return Err(ParserError::Unimplemented),
                // Operator::TableSize {
                //     table,
                // } => return Err(ParserError::Unimplemented),
            
                // 0xFE operators
                // https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md
                // Operator::MemoryAtomicNotify {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::MemoryAtomicWait32 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::MemoryAtomicWait64 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::AtomicFence {
                //     flags: u8,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicLoad {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicLoad {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicLoad8U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicLoad16U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicLoad8U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicLoad16U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicLoad32U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicStore {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicStore {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicStore8 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicStore16 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicStore8 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicStore16 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicStore32 {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwAdd {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwAdd {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8AddU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16AddU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8AddU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16AddU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32AddU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwSub {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwSub {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8SubU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16SubU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8SubU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16SubU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32SubU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwAnd {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwAnd {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8AndU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16AndU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8AndU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16AndU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32AndU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwOr {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwOr {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8OrU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16OrU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8OrU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16OrU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32OrU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwXor {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwXor {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8XorU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16XorU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8XorU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16XorU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32XorU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwXchg {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwXchg {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8XchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16XchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8XchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16XchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32XchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmwCmpxchg {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmwCmpxchg {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw8CmpxchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32AtomicRmw16CmpxchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw8CmpxchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw16CmpxchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64AtomicRmw32CmpxchgU {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
            
                // 0xFD operators
                // SIMD https://webassembly.github.io/simd/core/binary/instructions.html
                // Operator::V128Load {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load8x8S {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load8x8U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load16x4S {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load16x4U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load32x2S {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load32x2U {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load8Splat {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load16Splat {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load32Splat {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load64Splat {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load32Zero {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load64Zero {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Store {
                //     memarg,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load8Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load16Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load32Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Load64Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Store8Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Store16Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Store32Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Store64Lane {
                //     memarg,
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::V128Const {
                //     value: V128,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I8x16Shuffle {
                //     lanes,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I8x16ExtractLaneS {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I8x16ExtractLaneU {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I8x16ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtractLaneS {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtractLaneU {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I16x8ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtractLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I32x4ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtractLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I64x2ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::F32x4ExtractLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::F32x4ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::F64x2ExtractLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::F64x2ReplaceLane {
                //     lane,
                // } => return Err(ParserError::Unimplemented),
                // Operator::I8x16Swizzle => return Err(ParserError::Unimplemented),
                // Operator::I8x16Splat => return Err(ParserError::Unimplemented),
                // Operator::I16x8Splat => return Err(ParserError::Unimplemented),
                // Operator::I32x4Splat => return Err(ParserError::Unimplemented),
                // Operator::I64x2Splat => return Err(ParserError::Unimplemented),
                // Operator::F32x4Splat => return Err(ParserError::Unimplemented),
                // Operator::F64x2Splat => return Err(ParserError::Unimplemented),
                // Operator::I8x16Eq => return Err(ParserError::Unimplemented),
                // Operator::I8x16Ne => return Err(ParserError::Unimplemented),
                // Operator::I8x16LtS => return Err(ParserError::Unimplemented),
                // Operator::I8x16LtU => return Err(ParserError::Unimplemented),
                // Operator::I8x16GtS => return Err(ParserError::Unimplemented),
                // Operator::I8x16GtU => return Err(ParserError::Unimplemented),
                // Operator::I8x16LeS => return Err(ParserError::Unimplemented),
                // Operator::I8x16LeU => return Err(ParserError::Unimplemented),
                // Operator::I8x16GeS => return Err(ParserError::Unimplemented),
                // Operator::I8x16GeU => return Err(ParserError::Unimplemented),
                // Operator::I16x8Eq => return Err(ParserError::Unimplemented),
                // Operator::I16x8Ne => return Err(ParserError::Unimplemented),
                // Operator::I16x8LtS => return Err(ParserError::Unimplemented),
                // Operator::I16x8LtU => return Err(ParserError::Unimplemented),
                // Operator::I16x8GtS => return Err(ParserError::Unimplemented),
                // Operator::I16x8GtU => return Err(ParserError::Unimplemented),
                // Operator::I16x8LeS => return Err(ParserError::Unimplemented),
                // Operator::I16x8LeU => return Err(ParserError::Unimplemented),
                // Operator::I16x8GeS => return Err(ParserError::Unimplemented),
                // Operator::I16x8GeU => return Err(ParserError::Unimplemented),
                // Operator::I32x4Eq => return Err(ParserError::Unimplemented),
                // Operator::I32x4Ne => return Err(ParserError::Unimplemented),
                // Operator::I32x4LtS => return Err(ParserError::Unimplemented),
                // Operator::I32x4LtU => return Err(ParserError::Unimplemented),
                // Operator::I32x4GtS => return Err(ParserError::Unimplemented),
                // Operator::I32x4GtU => return Err(ParserError::Unimplemented),
                // Operator::I32x4LeS => return Err(ParserError::Unimplemented),
                // Operator::I32x4LeU => return Err(ParserError::Unimplemented),
                // Operator::I32x4GeS => return Err(ParserError::Unimplemented),
                // Operator::I32x4GeU => return Err(ParserError::Unimplemented),
                // Operator::I64x2Eq => return Err(ParserError::Unimplemented),
                // Operator::I64x2Ne => return Err(ParserError::Unimplemented),
                // Operator::I64x2LtS => return Err(ParserError::Unimplemented),
                // Operator::I64x2GtS => return Err(ParserError::Unimplemented),
                // Operator::I64x2LeS => return Err(ParserError::Unimplemented),
                // Operator::I64x2GeS => return Err(ParserError::Unimplemented),
                // Operator::F32x4Eq => return Err(ParserError::Unimplemented),
                // Operator::F32x4Ne => return Err(ParserError::Unimplemented),
                // Operator::F32x4Lt => return Err(ParserError::Unimplemented),
                // Operator::F32x4Gt => return Err(ParserError::Unimplemented),
                // Operator::F32x4Le => return Err(ParserError::Unimplemented),
                // Operator::F32x4Ge => return Err(ParserError::Unimplemented),
                // Operator::F64x2Eq => return Err(ParserError::Unimplemented),
                // Operator::F64x2Ne => return Err(ParserError::Unimplemented),
                // Operator::F64x2Lt => return Err(ParserError::Unimplemented),
                // Operator::F64x2Gt => return Err(ParserError::Unimplemented),
                // Operator::F64x2Le => return Err(ParserError::Unimplemented),
                // Operator::F64x2Ge => return Err(ParserError::Unimplemented),
                // Operator::V128Not => return Err(ParserError::Unimplemented),
                // Operator::V128And => return Err(ParserError::Unimplemented),
                // Operator::V128AndNot => return Err(ParserError::Unimplemented),
                // Operator::V128Or => return Err(ParserError::Unimplemented),
                // Operator::V128Xor => return Err(ParserError::Unimplemented),
                // Operator::V128Bitselect => return Err(ParserError::Unimplemented),
                // Operator::V128AnyTrue => return Err(ParserError::Unimplemented),
                // Operator::I8x16Abs => return Err(ParserError::Unimplemented),
                // Operator::I8x16Neg => return Err(ParserError::Unimplemented),
                // Operator::I8x16Popcnt => return Err(ParserError::Unimplemented),
                // Operator::I8x16AllTrue => return Err(ParserError::Unimplemented),
                // Operator::I8x16Bitmask => return Err(ParserError::Unimplemented),
                // Operator::I8x16NarrowI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I8x16NarrowI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I8x16Shl => return Err(ParserError::Unimplemented),
                // Operator::I8x16ShrS => return Err(ParserError::Unimplemented),
                // Operator::I8x16ShrU => return Err(ParserError::Unimplemented),
                // Operator::I8x16Add => return Err(ParserError::Unimplemented),
                // Operator::I8x16AddSatS => return Err(ParserError::Unimplemented),
                // Operator::I8x16AddSatU => return Err(ParserError::Unimplemented),
                // Operator::I8x16Sub => return Err(ParserError::Unimplemented),
                // Operator::I8x16SubSatS => return Err(ParserError::Unimplemented),
                // Operator::I8x16SubSatU => return Err(ParserError::Unimplemented),
                // Operator::I8x16MinS => return Err(ParserError::Unimplemented),
                // Operator::I8x16MinU => return Err(ParserError::Unimplemented),
                // Operator::I8x16MaxS => return Err(ParserError::Unimplemented),
                // Operator::I8x16MaxU => return Err(ParserError::Unimplemented),
                // Operator::I8x16RoundingAverageU => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtAddPairwiseI8x16S => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtAddPairwiseI8x16U => return Err(ParserError::Unimplemented),
                // Operator::I16x8Abs => return Err(ParserError::Unimplemented),
                // Operator::I16x8Neg => return Err(ParserError::Unimplemented),
                // Operator::I16x8Q15MulrSatS => return Err(ParserError::Unimplemented),
                // Operator::I16x8AllTrue => return Err(ParserError::Unimplemented),
                // Operator::I16x8Bitmask => return Err(ParserError::Unimplemented),
                // Operator::I16x8NarrowI32x4S => return Err(ParserError::Unimplemented),
                // Operator::I16x8NarrowI32x4U => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtendLowI8x16S => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtendHighI8x16S => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtendLowI8x16U => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtendHighI8x16U => return Err(ParserError::Unimplemented),
                // Operator::I16x8Shl => return Err(ParserError::Unimplemented),
                // Operator::I16x8ShrS => return Err(ParserError::Unimplemented),
                // Operator::I16x8ShrU => return Err(ParserError::Unimplemented),
                // Operator::I16x8Add => return Err(ParserError::Unimplemented),
                // Operator::I16x8AddSatS => return Err(ParserError::Unimplemented),
                // Operator::I16x8AddSatU => return Err(ParserError::Unimplemented),
                // Operator::I16x8Sub => return Err(ParserError::Unimplemented),
                // Operator::I16x8SubSatS => return Err(ParserError::Unimplemented),
                // Operator::I16x8SubSatU => return Err(ParserError::Unimplemented),
                // Operator::I16x8Mul => return Err(ParserError::Unimplemented),
                // Operator::I16x8MinS => return Err(ParserError::Unimplemented),
                // Operator::I16x8MinU => return Err(ParserError::Unimplemented),
                // Operator::I16x8MaxS => return Err(ParserError::Unimplemented),
                // Operator::I16x8MaxU => return Err(ParserError::Unimplemented),
                // Operator::I16x8RoundingAverageU => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtMulLowI8x16S => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtMulHighI8x16S => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtMulLowI8x16U => return Err(ParserError::Unimplemented),
                // Operator::I16x8ExtMulHighI8x16U => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtAddPairwiseI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtAddPairwiseI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I32x4Abs => return Err(ParserError::Unimplemented),
                // Operator::I32x4Neg => return Err(ParserError::Unimplemented),
                // Operator::I32x4AllTrue => return Err(ParserError::Unimplemented),
                // Operator::I32x4Bitmask => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtendLowI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtendHighI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtendLowI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtendHighI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I32x4Shl => return Err(ParserError::Unimplemented),
                // Operator::I32x4ShrS => return Err(ParserError::Unimplemented),
                // Operator::I32x4ShrU => return Err(ParserError::Unimplemented),
                // Operator::I32x4Add => return Err(ParserError::Unimplemented),
                // Operator::I32x4Sub => return Err(ParserError::Unimplemented),
                // Operator::I32x4Mul => return Err(ParserError::Unimplemented),
                // Operator::I32x4MinS => return Err(ParserError::Unimplemented),
                // Operator::I32x4MinU => return Err(ParserError::Unimplemented),
                // Operator::I32x4MaxS => return Err(ParserError::Unimplemented),
                // Operator::I32x4MaxU => return Err(ParserError::Unimplemented),
                // Operator::I32x4DotI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtMulLowI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtMulHighI16x8S => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtMulLowI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I32x4ExtMulHighI16x8U => return Err(ParserError::Unimplemented),
                // Operator::I64x2Abs => return Err(ParserError::Unimplemented),
                // Operator::I64x2Neg => return Err(ParserError::Unimplemented),
                // Operator::I64x2AllTrue => return Err(ParserError::Unimplemented),
                // Operator::I64x2Bitmask => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtendLowI32x4S => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtendHighI32x4S => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtendLowI32x4U => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtendHighI32x4U => return Err(ParserError::Unimplemented),
                // Operator::I64x2Shl => return Err(ParserError::Unimplemented),
                // Operator::I64x2ShrS => return Err(ParserError::Unimplemented),
                // Operator::I64x2ShrU => return Err(ParserError::Unimplemented),
                // Operator::I64x2Add => return Err(ParserError::Unimplemented),
                // Operator::I64x2Sub => return Err(ParserError::Unimplemented),
                // Operator::I64x2Mul => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtMulLowI32x4S => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtMulHighI32x4S => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtMulLowI32x4U => return Err(ParserError::Unimplemented),
                // Operator::I64x2ExtMulHighI32x4U => return Err(ParserError::Unimplemented),
                // Operator::F32x4Ceil => return Err(ParserError::Unimplemented),
                // Operator::F32x4Floor => return Err(ParserError::Unimplemented),
                // Operator::F32x4Trunc => return Err(ParserError::Unimplemented),
                // Operator::F32x4Nearest => return Err(ParserError::Unimplemented),
                // Operator::F32x4Abs => return Err(ParserError::Unimplemented),
                // Operator::F32x4Neg => return Err(ParserError::Unimplemented),
                // Operator::F32x4Sqrt => return Err(ParserError::Unimplemented),
                // Operator::F32x4Add => return Err(ParserError::Unimplemented),
                // Operator::F32x4Sub => return Err(ParserError::Unimplemented),
                // Operator::F32x4Mul => return Err(ParserError::Unimplemented),
                // Operator::F32x4Div => return Err(ParserError::Unimplemented),
                // Operator::F32x4Min => return Err(ParserError::Unimplemented),
                // Operator::F32x4Max => return Err(ParserError::Unimplemented),
                // Operator::F32x4PMin => return Err(ParserError::Unimplemented),
                // Operator::F32x4PMax => return Err(ParserError::Unimplemented),
                // Operator::F64x2Ceil => return Err(ParserError::Unimplemented),
                // Operator::F64x2Floor => return Err(ParserError::Unimplemented),
                // Operator::F64x2Trunc => return Err(ParserError::Unimplemented),
                // Operator::F64x2Nearest => return Err(ParserError::Unimplemented),
                // Operator::F64x2Abs => return Err(ParserError::Unimplemented),
                // Operator::F64x2Neg => return Err(ParserError::Unimplemented),
                // Operator::F64x2Sqrt => return Err(ParserError::Unimplemented),
                // Operator::F64x2Add => return Err(ParserError::Unimplemented),
                // Operator::F64x2Sub => return Err(ParserError::Unimplemented),
                // Operator::F64x2Mul => return Err(ParserError::Unimplemented),
                // Operator::F64x2Div => return Err(ParserError::Unimplemented),
                // Operator::F64x2Min => return Err(ParserError::Unimplemented),
                // Operator::F64x2Max => return Err(ParserError::Unimplemented),
                // Operator::F64x2PMin => return Err(ParserError::Unimplemented),
                // Operator::F64x2PMax => return Err(ParserError::Unimplemented),
                // Operator::I32x4TruncSatF32x4S => return Err(ParserError::Unimplemented),
                // Operator::I32x4TruncSatF32x4U => return Err(ParserError::Unimplemented),
                // Operator::F32x4ConvertI32x4S => return Err(ParserError::Unimplemented),
                // Operator::F32x4ConvertI32x4U => return Err(ParserError::Unimplemented),
                // Operator::I32x4TruncSatF64x2SZero => return Err(ParserError::Unimplemented),
                // Operator::I32x4TruncSatF64x2UZero => return Err(ParserError::Unimplemented),
                // Operator::F64x2ConvertLowI32x4S => return Err(ParserError::Unimplemented),
                // Operator::F64x2ConvertLowI32x4U => return Err(ParserError::Unimplemented),
                // Operator::F32x4DemoteF64x2Zero => return Err(ParserError::Unimplemented),
                // Operator::F64x2PromoteLowF32x4 => return Err(ParserError::Unimplemented),
                // Operator::I8x16RelaxedSwizzle => return Err(ParserError::Unimplemented),
                // Operator::I32x4RelaxedTruncSatF32x4S => return Err(ParserError::Unimplemented),
                // Operator::I32x4RelaxedTruncSatF32x4U => return Err(ParserError::Unimplemented),
                // Operator::I32x4RelaxedTruncSatF64x2SZero => return Err(ParserError::Unimplemented),
                // Operator::I32x4RelaxedTruncSatF64x2UZero => return Err(ParserError::Unimplemented),
                // Operator::F32x4Fma => return Err(ParserError::Unimplemented),
                // Operator::F32x4Fms => return Err(ParserError::Unimplemented),
                // Operator::F64x2Fma => return Err(ParserError::Unimplemented),
                // Operator::F64x2Fms => return Err(ParserError::Unimplemented),
                // Operator::I8x16LaneSelect => return Err(ParserError::Unimplemented),
                // Operator::I16x8LaneSelect => return Err(ParserError::Unimplemented),
                // Operator::I32x4LaneSelect => return Err(ParserError::Unimplemented),
                // Operator::I64x2LaneSelect => return Err(ParserError::Unimplemented),
                // Operator::F32x4RelaxedMin => return Err(ParserError::Unimplemented),
                // Operator::F32x4RelaxedMax => return Err(ParserError::Unimplemented),
                // Operator::F64x2RelaxedMin => return Err(ParserError::Unimplemented),
                // Operator::F64x2RelaxedMax => return Err(ParserError::Unimplemented),
                o => return Err(ParserError::<'a>::Unimplemented(o))
            }
        } else {
            break
        }
    }

    if exprs.len() == 1 {
        if let Some(expr) = exprs.pop() {
            stmts.push(Statement::Unassigned(expr))
        }
    }

    Ok(stmts)
}

#[derive(Debug, Clone)]
pub enum ParserError<'a> {
    Invalid {
        expressions: Vec<Expression>,
        statements: Vec<Statement>,
        operator: Operator<'a>,
    },
    Unimplemented(Operator<'a>)
}
