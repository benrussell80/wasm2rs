use crate::wasm_type::WASMType;
use wasmparser;


#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Vec<WASMType>,
    pub returns: Vec<WASMType>,
}

impl From<wasmparser::FuncType> for FuncType {
    fn from(obj: wasmparser::FuncType) -> Self {
        Self {
            params: obj.params.into_iter().map(|p| p.into()).collect(),
            returns: obj.returns.into_iter().map(|p| p.into()).collect(),
        }
    }
}
