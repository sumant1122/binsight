use object::{Object, ObjectSection, ObjectSymbol};
use std::path::Path;
use memmap2::Mmap;
use std::fs::File;
use rustc_demangle::demangle;

pub struct BinaryInfo {
    pub total_size: u64,
    pub sections: Vec<SectionInfo>,
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Clone)]
pub struct SectionInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub demangled_name: String,
    pub size: u64,
    pub crate_name: Option<String>,
}

pub fn load_and_analyze(path: &Path) -> anyhow::Result<BinaryInfo> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let object = object::File::parse(&*mmap)?;

    let total_size = mmap.len() as u64;
    let mut sections = Vec::new();

    for section in object.sections() {
        sections.push(SectionInfo {
            name: section.name()?.to_string(),
            size: section.size(),
        });
    }

    let mut symbols = Vec::new();
    for symbol in object.symbols() {
        if symbol.size() > 0 {
            let name = symbol.name()?.to_string();
            let demangled = demangle(&name).to_string();
            
            // Simple heuristic for crate name from demangled Rust symbol
            // e.g., binsize::main::he0da... -> binsize
            let crate_name = if demangled.contains("::") {
                Some(demangled.split("::").next().unwrap().to_string())
            } else {
                None
            };

            symbols.push(SymbolInfo {
                name,
                demangled_name: demangled,
                size: symbol.size(),
                crate_name,
            });
        }
    }

    Ok(BinaryInfo {
        total_size,
        sections,
        symbols,
    })
}
