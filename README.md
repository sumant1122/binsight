# 📊 binsize

`binsize` is a production-quality binary size analyzer designed for developers. It helps you understand exactly why your binaries are large by breaking them down by section, crate, and symbol, and provides actionable suggestions for optimization.

## ✨ Features

- **Analyze Binary**: Get a high-level breakdown of executable sections (`.text`, `.data`, `.debug`, etc.).
- **Top Contributors**: Identify which crates or modules are consuming the most space.
- **Diff Mode**: Compare two binaries to see exactly what changed between builds.
- **Suggestions Engine**: Automatically detect common size issues (like debug symbols in release builds) and get optimization tips.
- **Cross-Platform**: Supports ELF (Linux), Mach-O (macOS), and PE (Windows).

## 🚀 Installation

Ensure you have Rust and Cargo installed, then clone the repository and build:

```bash
git clone https://github.com/youruser/binsize.git
cd binsize
cargo install --path .
```

## 🛠 Usage

### 1. Basic Analysis
Show the size contribution of each binary section:
```bash
binsize analyze target/release/my_app
```

### 2. Find Top Contributors
Identify the largest crates and symbols:
```bash
binsize top target/release/my_app
```

### 3. Compare Binaries (Diff)
See what changed between two versions of a binary:
```bash
binsize diff old_binary new_binary
```

## 💡 Examples

### Analysis Output
```text
📊 binsize Analysis
Total Size: 2.45 MB

╭────────────────────┬───────────┬───────╮
│ Section            ┆ Size      ┆ %     │
╞════════════════════╪═══════════╪═══════╡
│ .text              ┆ 1.84 MB   ┆ 75.1% │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ .rodata            ┆ 400.12 KB ┆ 16.3% │
╰────────────────────┴───────────┴───────╯

💡 Optimization Suggestions
• Strip Debug Symbols: Your binary contains debug symbols. Use 'strip' or 'cargo build --release'.
```

### Top Contributors
```text
🔥 Top Contributors (Grouped by Crate/Module)
╭──────────────────┬───────────┬──────╮
│ Crate/Module     ┆ Size      ┆ %    │
╞══════════════════╪═══════════╪══════╡
│ std              ┆ 1.20 MB   ┆ 48.9%│
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
│ regex            ┆ 450.00 KB ┆ 18.3%│
╰──────────────────┴───────────┴──────╯
```

## ⚖️ License
MIT
