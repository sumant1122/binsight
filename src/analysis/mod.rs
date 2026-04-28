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

pub struct Diagnostic {
    pub category: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
}

pub enum Severity {
    Info,
    Warning,
}

pub fn run_diagnostics(info: &BinaryInfo) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    // 1. Strip Check
    if info.sections.iter().any(|s| s.name.contains(".debug_info")) {
        diags.push(Diagnostic {
            category: "Binary".to_string(),
            title: "Unstripped Binary".to_string(),
            description: "Debug symbols found. Run 'strip' or 'cargo build --release' to reduce size by 60-80%.".to_string(),
            severity: Severity::Warning,
        });
    }

    // 2. Generic Bloat Check
    let mut generics: std::collections::HashMap<String, (usize, u64)> = std::collections::HashMap::new();
    for sym in &info.symbols {
        let base_name = if let Some(idx) = sym.demangled_name.rfind("::h") {
            if sym.demangled_name[idx..].len() >= 18 { // ::h + 16 hex chars
                &sym.demangled_name[..idx]
            } else {
                &sym.demangled_name
            }
        } else {
            &sym.demangled_name
        };
        
        let entry = generics.entry(base_name.to_string()).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += sym.size;
    }

    let mut high_bloat_generics: Vec<_> = generics.into_iter()
        .filter(|(_, (count, _))| *count > 5)
        .collect();
    high_bloat_generics.sort_by_key(|b| std::cmp::Reverse(b.1.1));

    if let Some((name, (count, size))) = high_bloat_generics.first() {
        diags.push(Diagnostic {
            category: "Generics".to_string(),
            title: "High Monomorphization Bloat".to_string(),
            description: format!(
                "Symbol '{}' is instantiated {} times, taking {}. Consider dynamic dispatch (trait objects).", 
                name, count, crate::ui::format_size(*size)
            ),
            severity: Severity::Warning,
        });
    }

    // 3. Panic machinery check
    let panic_size: u64 = info.symbols.iter()
        .filter(|s| s.demangled_name.contains("panic") || s.demangled_name.contains("begin_unwind"))
        .map(|s| s.size)
        .sum();

    if panic_size > 50_000 {
        diags.push(Diagnostic {
            category: "Runtime".to_string(),
            title: "Panic Machinery Bloat".to_string(),
            description: format!(
                "Panic handling takes {}. Consider 'panic = \"abort\"' in Cargo.toml to prune this.",
                crate::ui::format_size(panic_size)
            ),
            severity: Severity::Info,
        });
    }

    // 4. Large Read-Only Data
    let rodata_size = info.sections.iter()
        .find(|s| s.name == ".rodata")
        .map(|s| s.size)
        .unwrap_or(0);
    
    if rodata_size > info.total_size / 4 && info.total_size > 1_000_000 {
        diags.push(Diagnostic {
            category: "Data".to_string(),
            title: "Heavy Read-Only Data".to_string(),
            description: "More than 25% of your binary is read-only data. Check for large embedded assets or strings.".to_string(),
            severity: Severity::Info,
        });
    }

    // 5. Large Individual Symbols
    let mut large_symbols: Vec<_> = info.symbols.iter()
        .filter(|s| s.size > 100_000)
        .collect();
    large_symbols.sort_by_key(|b| std::cmp::Reverse(b.size));

    if let Some(sym) = large_symbols.first() {
        diags.push(Diagnostic {
            category: "Code".to_string(),
            title: "Extremely Large Symbol".to_string(),
            description: format!(
                "Symbol '{}' is {}, which is very large for a single function. Consider splitting it.",
                sym.demangled_name, crate::ui::format_size(sym.size)
            ),
            severity: Severity::Info,
        });
    }

    diags
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::{BinaryInfo, SectionInfo, SymbolInfo};

    fn mock_info() -> BinaryInfo {
        BinaryInfo {
            total_size: 1000,
            sections: vec![SectionInfo { name: ".text".to_string(), size: 500 }],
            symbols: vec![SymbolInfo {
                demangled_name: "test_sym".to_string(),
                size: 100,
                module_path: vec!["crate".to_string()],
                file: None,
                line: None,
            }],
        }
    }

    #[test]
    fn test_strip_diagnostic() {
        let mut info = mock_info();
        info.sections.push(SectionInfo { name: ".debug_info".to_string(), size: 100 });
        let diags = run_diagnostics(&info);
        assert!(diags.iter().any(|d| d.title == "Unstripped Binary"));
    }

    #[test]
    fn test_panic_diagnostic() {
        let mut info = mock_info();
        info.symbols.push(SymbolInfo {
            demangled_name: "rust_begin_unwind".to_string(),
            size: 60000,
            module_path: vec![],
            file: None,
            line: None,
        });
        let diags = run_diagnostics(&info);
        assert!(diags.iter().any(|d| d.title == "Panic Machinery Bloat"));
    }
}
