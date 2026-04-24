# 📊 binsight

`binsight` is a production-quality binary size analyzer designed for developers. It helps you understand exactly why your binaries are large by breaking them down by section, crate, and symbol, and provides actionable suggestions for optimization.

## ✨ Features

- **Analyze Binary**: Get a high-level breakdown of executable sections (`.text`, `.data`, `.debug`, etc.).
- **Top Contributors**: Identify which crates or modules are consuming the most space. Supports deep drill-down with `--depth`.
- **Interactive Explorer (TUI)**: Navigate your binary's size distribution interactively in the terminal.
- **Diff Mode**: Compare two binaries to see exactly what changed between builds.
- **Binary Diagnosis**: Automatically detect common size issues (monomorphization bloat, panic machinery, unstripped symbols).
- **Source Mapping**: Link large symbols directly to source files and line numbers.
- **Cross-Platform**: Supports ELF (Linux), Mach-O (macOS), and PE (Windows).

## 🚀 Installation

Ensure you have Rust and Cargo installed, then clone the repository and build:

```bash
git clone https://github.com/youruser/binsight.git
cd binsight
cargo install --path .
```

## 🛠 Usage

### 1. Basic Analysis
Show the size contribution of each binary section:
```bash
binsight analyze target/release/my_app
```

### 2. Interactive Explorer
Launch the TUI to explore size distribution:
```bash
binsight explore target/release/my_app
```

### 3. Find Top Contributors
Identify the largest crates and symbols. Use `--depth` to see sub-modules:
```bash
# Crate level
binsight top target/release/my_app --depth 1

# Module level (drill-down)
binsight top target/release/my_app --depth 3
```

### 4. Binary Diagnosis
Run detailed health checks to find bloat:
```bash
binsight diagnose target/release/my_app
```

### 5. Compare Binaries (Diff)
See what changed between two versions of a binary:
```bash
binsight diff old_binary new_binary
```

## 💡 Output Examples

### 📊 Analyze Output
```text
📊 binsight Analysis
Total Size: 2.45 MB

╭────────────────────┬───────────┬───────╮
│ Section            ┆ Size      ┆ %     │
╞════════════════════╪═══════════╪═══════╡
│ .text              ┆ 1.84 MB   ┆ 75.1% │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ .rodata            ┆ 400.12 KB ┆ 16.3% │
╰────────────────────┴───────────┴───────╯

🩺 Binary Health Diagnosis
╭──────────┬───────────────────┬─────────────────────────────────────────────╮
│ Category ┆ Issue             ┆ Recommendation                              │
╞══════════╪═══════════════════╪═════════════════════════════════════════════╡
│ Binary   ┆ Unstripped Binary ┆ Run 'strip' to reduce size by 60-80%.       │
╰──────────┴───────────────────┴─────────────────────────────────────────────╯
```

### 🔥 Top Contributors (with Source Mapping)
```text
🔥 Top Contributors (Grouped by Depth 1)
╭────────────────┬───────────┬──────╮
│ Crate/Module   ┆ Size      ┆ %    │
╞════════════════╪═══════════╪══════╡
│ core           ┆ 845.15 KB ┆ 2.3% │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
│ clap_builder   ┆ 302.08 KB ┆ 0.8% │
╰────────────────┴───────────┴──────╯

📜 Largest Symbols
╭───────────────────────────┬─────────────────────┬──────────┬──────╮
│ Symbol                    ┆ Location            ┆ Size     ┆ %    │
╞═══════════════════════════╪═════════════════════╪══════════╪══════╡
│ clap_builder::parser      ┆ parser.rs:77        ┆ 15.50 KB ┆ 0.1% │
╰───────────────────────────┴─────────────────────┴──────────┴──────╯
```

### 🔍 Diff Mode
```text
🔍 Binary Comparison
Old Size: 22.39 MB
New Size: 3.10 MB
Delta:    19.29 MB (Saved!)
```

## ⚖️ License
MIT
