use crate::binary::BinaryInfo;
use std::collections::HashMap;

pub struct DiffResult {
    pub old_size: u64,
    pub new_size: u64,
    pub section_diffs: Vec<SectionDiff>,
    pub symbol_diffs: Vec<SymbolDiff>,
}

#[derive(Clone)]
pub struct SectionDiff {
    pub name: String,
    pub old_size: u64,
    pub new_size: u64,
}

#[derive(Clone)]
pub struct SymbolDiff {
    pub name: String,
    pub old_size: u64,
    pub new_size: u64,
}

pub struct Suggestion {
    pub title: String,
    pub description: String,
}

pub fn get_suggestions(info: &BinaryInfo) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    // Check for debug symbols
    if info.sections.iter().any(|s| s.name.contains(".debug_info")) {
        suggestions.push(Suggestion {
            title: "Strip Debug Symbols".to_string(),
            description: "Your binary contains debug symbols. Use 'strip' or 'cargo build --release' to reduce size significantly.".to_string(),
        });
    }

    // Check if it's a debug build (heuristic: large std/core symbols)
    let text_size = info.sections.iter().find(|s| s.name == ".text").map(|s| s.size).unwrap_or(0);
    
    if text_size > 1_000_000 && info.sections.iter().any(|s| s.name.contains(".debug")) {
         suggestions.push(Suggestion {
            title: "Use Release Mode".to_string(),
            description: "The binary seems to be built in debug mode. Build with '--release' for a much smaller and faster binary.".to_string(),
        });
    }

    // Check for lto suggestion
    if text_size > 5_000_000 {
        suggestions.push(Suggestion {
            title: "Enable LTO".to_string(),
            description: "For large binaries, enabling Link Time Optimization (LTO) in your Cargo.toml can prune unused code across crates.".to_string(),
        });
    }

    suggestions
}

pub fn compare(old: &BinaryInfo, new: &BinaryInfo) -> DiffResult {
    let mut section_diffs = Vec::new();
    let mut old_sections: HashMap<String, u64> = old.sections.iter().map(|s| (s.name.clone(), s.size)).collect();
    let mut new_sections: HashMap<String, u64> = new.sections.iter().map(|s| (s.name.clone(), s.size)).collect();

    let all_section_names: std::collections::HashSet<_> = old_sections.keys().chain(new_sections.keys()).cloned().collect();

    for name in all_section_names {
        let old_size = old_sections.remove(&name).unwrap_or(0);
        let new_size = new_sections.remove(&name).unwrap_or(0);
        if old_size != new_size {
            section_diffs.push(SectionDiff { name, old_size, new_size });
        }
    }

    let mut symbol_diffs = Vec::new();
    let mut old_symbols: HashMap<String, u64> = old.symbols.iter().map(|s| (s.demangled_name.clone(), s.size)).collect();
    let mut new_symbols: HashMap<String, u64> = new.symbols.iter().map(|s| (s.demangled_name.clone(), s.size)).collect();

    let all_symbol_names: std::collections::HashSet<_> = old_symbols.keys().chain(new_symbols.keys()).cloned().collect();

    for name in all_symbol_names {
        let old_size = old_symbols.remove(&name).unwrap_or(0);
        let new_size = new_symbols.remove(&name).unwrap_or(0);
        if old_size != new_size {
            symbol_diffs.push(SymbolDiff { name, old_size, new_size });
        }
    }

    DiffResult {
        old_size: old.total_size,
        new_size: new.total_size,
        section_diffs,
        symbol_diffs,
    }
}
