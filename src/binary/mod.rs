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
    pub demangled_name: String,
    pub size: u64,
    pub module_path: Vec<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

pub fn load_and_analyze(path: &Path) -> anyhow::Result<BinaryInfo> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let object = object::File::parse(&*mmap)?;

    // Source mapping context using 0.21 API (requires object feature)
    let ctx = addr2line::Context::new(&object).ok();

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
            
            // Refined heuristic for crate and module path
            // Rust symbols often end with a hash like ::h1234567890abcdef
            let base_name = if let Some(idx) = demangled.rfind("::h") {
                if demangled[idx..].len() >= 10 { // conservative check for hash
                    &demangled[..idx]
                } else {
                    &demangled
                }
            } else {
                &demangled
            };

            let parts: Vec<String> = base_name.split("::")
                .map(|s| s.to_string())
                .collect();
            
            let module_path = if parts.len() > 1 {
                parts[..parts.len()-1].to_vec()
            } else {
                // For top-level symbols, the first part might be the crate
                // but if there are no colons, we don't have enough info to be sure.
                Vec::new()
            };

            let mut source_file = None;
            let mut source_line = None;

            if let Some(ctx) = &ctx {
                if let Ok(mut frames) = ctx.find_frames(symbol.address()).skip_all_loads() {
                    if let Ok(Some(frame)) = frames.next() {
                        if let Some(location) = frame.location {
                            source_file = location.file.map(|f: &str| f.to_string());
                            source_line = location.line;
                        }
                    }
                }
            }

            symbols.push(SymbolInfo {
                demangled_name: demangled,
                size: symbol.size(),
                module_path,
                file: source_file,
                line: source_line,
            });
        }
    }

    Ok(BinaryInfo {
        total_size,
        sections,
        symbols,
    })
}
