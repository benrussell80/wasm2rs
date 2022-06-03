use wasmparser::{Parser, Payload};
use crate::context::Context;


pub fn parse(data: &[u8]) -> Context {
    let parser = Parser::new(0);

    let mut c = Context::builder();

    for result in parser.parse_all(data) {
        if let Ok(payload) = result {
            match payload {
                Payload::FunctionSection(funcs) => c = c.set_funcs(funcs),
                Payload::TypeSection(types) => c = c.set_types(types),
                Payload::ImportSection(imports) => c = c.set_imports(imports),
                Payload::CodeSectionEntry(body) => c = c.add_code_section(body),
                Payload::TableSection(_) => {},
                Payload::MemorySection(_) => {},
                Payload::GlobalSection(_) => {},
                Payload::ExportSection(exports) => c = c.set_exports(exports),
                Payload::StartSection { func, .. } => {
                    println!("Start Function is: {func}")
                },
                Payload::DataCountSection { .. } => {},
                Payload::DataSection(_) => {}
                Payload::ElementSection(_) => {}
                Payload::CustomSection(_)
                | Payload::Version { .. }
                | Payload::TagSection(_)
                | Payload::ComponentSection { .. } 
                | Payload::ComponentTypeSection(_)
                | Payload::ComponentImportSection(_)
                | Payload::ComponentFunctionSection(_)
                | Payload::ModuleSection { .. }   
                | Payload::InstanceSection(_)
                | Payload::ComponentExportSection(_)
                | Payload::ComponentStartSection(_)
                | Payload::UnknownSection { .. }
                | Payload::AliasSection(_)
                | Payload::CodeSectionStart { .. }
                | Payload::End(_)
                => { }
            }
        }
    }
    
    c.build()
}
