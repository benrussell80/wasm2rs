use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug, Copy, Clone)]
pub enum WASMType {
    I32,
    I64,
    F32,
    F64,
}

impl Display for WASMType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        })
    }
}

impl From<wasmparser::Type> for WASMType {
    fn from(obj: wasmparser::Type) -> Self {
        match obj {
            wasmparser::Type::I32 => Self::I32,
            wasmparser::Type::I64 => Self::I64,
            wasmparser::Type::F32 => Self::F32,
            wasmparser::Type::F64 => Self::F64,
            _ => unimplemented!()
        }
    }
}

impl From<&wasmparser::Type> for WASMType {
    fn from(obj: &wasmparser::Type) -> Self {
        match obj {
            wasmparser::Type::I32 => Self::I32,
            wasmparser::Type::I64 => Self::I64,
            wasmparser::Type::F32 => Self::F32,
            wasmparser::Type::F64 => Self::F64,
            _ => unimplemented!()
        }
    }
}