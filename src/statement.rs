use crate::expression::{Expression, LevelKind};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Statement {
    LocalSet(String, Expression),
    Return(Option<Expression>),
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
    Block(Vec<Statement>, u32),
    Loop(Vec<Statement>, u32),
    BrIf {
        cond: Expression,
        block_depth: u32,
        relative_depth: u32
    },
    Br {
        block_depth: u32,
        relative_depth: u32
    },
    BrTable {
        cond: Expression,
        stack: Vec<LevelKind>,
        table: Vec<u32>,
        default: u32,
    },
    Continue {
        block_depth: u32,
        relative_depth: u32,
    },
    ContinueIf {
        cond: Expression,
        block_depth: u32,
        relative_depth: u32
    },
    RawRust(Vec<String>),
}

// add support for indentation
// handle contextual meaning for br, br_if, and br_table between loop and block (continue vs break)

pub const INDENTATION: usize = 4;

impl Statement {
    pub fn emit_code(&self, indentation: usize) -> Vec<String> {
        let mut lines = vec![];
        match self {
            Self::LocalSet(index, expr) => lines.push(format!("{:indentation$}{} = {};", " ", index, expr.emit_code())),
            Self::Return(expr) => {
                match expr {
                    Some(e) => lines.push(format!("{:indentation$}return {};", " ", e.emit_code())),
                    None => lines.push(format!("{:indentation$}return;", " ")),
                }
            }
            Self::Unreachable => lines.push(format!("{:indentation$}unreachable!();", " ")),
            Self::Nop => lines.push(format!("{:indentation$};", " ")),
            Self::GlobalSet(index, expr) => lines.push(format!("{:indentation$}{} = {};", " ", index, expr.emit_code())),
            Self::Unassigned(expr) => lines.push(format!("{:indentation$}{}", " ", expr.emit_code())),
            Self::I32Store(ptr_expr, value_expr, align, offset) => {
                let method = if *align == 2 {
                    "write"
                } else {
                    "write_unaligned"
                };
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i32).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i32).cast::<u8>().add({offset}).cast::<i32>().{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i64).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i64).cast::<u8>().add({offset}).cast::<i64>().{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut f32).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut f32).cast::<u8>().add({offset}).cast::<f32>().{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut f64).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut f64).cast::<u8>().add({offset}).cast::<f64>().{method}({value_code});", " "))
                }
            },
            Self::I32Store8(ptr_expr, value_expr, _, offset) => {
                let method = "write";
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i8).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i8).add({offset}).{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i16).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i16).cast::<u8>().add({offset}).cast::<i16>().{method}({value_code});", " "))
                }
            },
            Self::I64Store8(ptr_expr, value_expr, _, offset) => {
                let method = "write";
                let ptr_code = ptr_expr.emit_code();
                let value_code = value_expr.emit_code();
                if *offset == 0 {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i8).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i8).add({offset}).{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i16).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i16).cast::<u8>().add({offset}).cast::<i16>().{method}({value_code});", " "))
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
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i32).{method}({value_code});", " "))
                } else {
                    lines.push(format!("{:indentation$}({ptr_code} as *mut i32).cast::<u8>().add({offset}).cast::<i32>().{method}({value_code});", " "))
                }
            },
            Self::Drop(expr) => {
                lines.push(format!(
                    "{:indentation$}drop({});",
                    " ",
                    expr.emit_code(),
                ))
            },
            Self::Block(stmts, depth) => {
                lines.push(format!("{:indentation$}'B{depth}: loop {{", " "));
                
                for stmt in stmts.iter() {
                    lines.extend(stmt.emit_code(indentation + INDENTATION));
                }

                lines.push(format!("{:indentation$}break;", " ", indentation=indentation+INDENTATION));
                lines.push(format!("{:indentation$}}};", " "));
            },
            Self::Loop(stmts, depth) => {
                lines.push(format!("{:indentation$}'B{depth}: loop {{", " "));
                
                for stmt in stmts.iter() {
                    lines.extend(stmt.emit_code(indentation + INDENTATION));
                }

                lines.push(format!("{:indentation$}}};", " "));
            },
            Self::BrIf { cond, block_depth, relative_depth } => {
                lines.push(format!("{:indentation$}if {} != 0 {{ break 'B{} }}", " ", cond.emit_code(), block_depth - relative_depth))
            },
            Self::Br { block_depth, relative_depth } => {
                lines.push(format!("{:indentation$}break 'B{};", " ", block_depth - relative_depth))
            },
            Self::BrTable { cond, stack, table, default } => {
                lines.push(format!("{:indentation$}match {} {{", " ", cond.emit_code()));

                for (i, relative_depth) in table.iter().enumerate() {
                    let instruction = {
                        let block_depth = stack.len() - 1;
                        match stack.get(block_depth - *relative_depth as usize) {
                            Some(LevelKind::Block) => "break",
                            Some(LevelKind::Loop) => "continue",
                            None => unreachable!()
                        }
                    };
                    lines.push(format!("{:indentation$}{i} => {instruction} 'B{},", " ", (stack.len() - 1) as u32 - relative_depth))
                }

                let instruction = {
                    let block_depth = stack.len() - 1;
                    match stack.get(block_depth - *default as usize) {
                        Some(LevelKind::Block) => "break",
                        Some(LevelKind::Loop) => "continue",
                        None => unreachable!()
                    }
                };

                lines.push(format!("{:indentation$}_ => {instruction} 'B{},", " ", (stack.len() - 1) as u32 - default));

                lines.push(format!("{:indentation$}}}", " "));
            },
            Self::Continue { block_depth, relative_depth } => {
                lines.push(format!("{:indentation$}continue 'B{}", " ", block_depth - relative_depth));
            },
            Self::ContinueIf { cond, block_depth, relative_depth } => {
                lines.push(format!("{:indentation$}if {} != 0 {{ continue 'B{} }}", " ", cond.emit_code(), block_depth - relative_depth));
            },
            Self::RawRust(raw_lines) => {
                for raw_line in raw_lines.iter() {
                    lines.push(format!("{:indentation$}{}", " ", raw_line));
                }
            }
        };
        lines
    }
}

