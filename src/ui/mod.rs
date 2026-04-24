pub mod tui;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use crate::binary::BinaryInfo;
use colored::*;

use crate::analysis::{DiffResult, Diagnostic, Severity};

pub fn display_diagnostics(diags: &[Diagnostic]) {
    if diags.is_empty() {
        println!("\n✨ {}", "No issues found. Your binary looks lean!".green().bold());
        return;
    }

    println!("\n🩺 {}", "Binary Health Diagnosis".cyan().bold());
    
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["Category", "Issue", "Recommendation"]);

    for d in diags {
        let title = match d.severity {
            Severity::Critical => d.title.red().bold(),
            Severity::Warning => d.title.yellow().bold(),
            Severity::Info => d.title.blue().bold(),
        };

        table.add_row(vec![
            comfy_table::Cell::new(d.category.clone()),
            comfy_table::Cell::new(title.to_string()),
            comfy_table::Cell::new(d.description.clone()),
        ]);
    }
    println!("{table}");
}

pub fn display_diff(diff: &DiffResult) {
    println!("\n🔍 {} Comparison", "Binary".magenta().bold());
    
    let total_delta = diff.new_size as i64 - diff.old_size as i64;
    let delta_color = if total_delta > 0 { "red" } else { "green" };
    
    println!("Old Size: {}", format_size(diff.old_size));
    println!("New Size: {}", format_size(diff.new_size));
    println!("Delta:    {}\n", format_size(total_delta.abs() as u64).color(delta_color).bold());

    if !diff.section_diffs.is_empty() {
        println!("📂 {} Changes", "Section".cyan());
        let mut table = Table::new();
        table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Section", "Old", "New", "Delta"]);

        let mut sorted_sections = diff.section_diffs.clone();
        sorted_sections.sort_by_key(|s| (s.new_size as i64 - s.old_size as i64).abs());
        sorted_sections.reverse();

        for s in sorted_sections.iter().take(10) {
            let delta = s.new_size as i64 - s.old_size as i64;
            let delta_str = if delta > 0 { 
                format!("+{}", format_size(delta as u64)).red() 
            } else { 
                format!("-{}", format_size(delta.abs() as u64)).green() 
            };

            table.add_row(vec![
                s.name.clone(),
                format_size(s.old_size),
                format_size(s.new_size),
                delta_str.to_string(),
            ]);
        }
        println!("{table}\n");
    }

    if !diff.symbol_diffs.is_empty() {
        println!("📜 {} Changes (Top 10)", "Symbol".yellow());
        let mut table = Table::new();
        table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Symbol", "Old", "New", "Delta"]);

        let mut sorted_symbols = diff.symbol_diffs.clone();
        sorted_symbols.sort_by_key(|s| (s.new_size as i64 - s.old_size as i64).abs());
        sorted_symbols.reverse();

        for s in sorted_symbols.iter().take(10) {
            let delta = s.new_size as i64 - s.old_size as i64;
            let delta_str = if delta > 0 { 
                format!("+{}", format_size(delta as u64)).red() 
            } else { 
                format!("-{}", format_size(delta.abs() as u64)).green() 
            };

            table.add_row(vec![
                s.name.clone(),
                format_size(s.old_size),
                format_size(s.new_size),
                delta_str.to_string(),
            ]);
        }
        println!("{table}");
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn display_analysis(info: &BinaryInfo) {
    println!("\n📊 {} Analysis", "binsize".cyan().bold());
    println!("Total Size: {}\n", format_size(info.total_size).green().bold());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Section", "Size", "%"]);

    let mut sorted_sections = info.sections.clone();
    sorted_sections.sort_by(|a, b| b.size.cmp(&a.size));

    for section in sorted_sections {
        if section.size == 0 { continue; }
        let percentage = (section.size as f64 / info.total_size as f64) * 100.0;
        table.add_row(vec![
            section.name,
            format_size(section.size),
            format!("{:.1}%", percentage),
        ]);
    }

    println!("{table}");
}

pub fn display_top_contributors(info: &BinaryInfo, depth: usize) {
    println!("\n🔥 {} Contributors (Grouped by Depth {})", "Top".red().bold(), depth);

    let mut group_sizes: std::collections::HashMap<String, u64> = std::collections::HashMap::new();

    for symbol in &info.symbols {
        let name = if symbol.module_path.is_empty() {
            "unknown".to_string()
        } else {
            let take_len = std::cmp::min(symbol.module_path.len(), depth);
            symbol.module_path[..take_len].join("::")
        };
        *group_sizes.entry(name).or_insert(0) += symbol.size;
    }

    let mut sorted_groups: Vec<_> = group_sizes.into_iter().collect();
    sorted_groups.sort_by(|a, b| b.1.cmp(&a.1));

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Crate/Module", "Size", "%"]);

    for (name, size) in sorted_groups.iter().take(20) {
        let percentage = (*size as f64 / info.total_size as f64) * 100.0;
        table.add_row(vec![
            name.to_string(),
            format_size(*size),
            format!("{:.1}%", percentage),
        ]);
    }

    println!("{table}");

    println!("\n📜 {} Symbols", "Largest".yellow().bold());
    let mut sorted_symbols = info.symbols.clone();
    sorted_symbols.sort_by(|a, b| b.size.cmp(&a.size));

    let mut sym_table = Table::new();
    sym_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Symbol", "Location", "Size", "%"]);

    for symbol in sorted_symbols.iter().take(10) {
        let percentage = (symbol.size as f64 / info.total_size as f64) * 100.0;
        let location = match (&symbol.file, symbol.line) {
            (Some(f), Some(l)) => {
                let filename = std::path::Path::new(f)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(f);
                format!("{}:{}", filename, l)
            }
            _ => "unknown".to_string(),
        };

        sym_table.add_row(vec![
            symbol.demangled_name.clone(),
            location,
            format_size(symbol.size),
            format!("{:.1}%", percentage),
        ]);
    }
    println!("{sym_table}");
}
