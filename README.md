# 📊 binsize

`binsize` is a production-quality binary size analyzer designed for developers. It helps you understand exactly why your binaries are large by breaking them down by section, crate, and symbol, and provides actionable suggestions for optimization.

## ✨ Features

- **Analyze Binary**: Get a high-level breakdown of executable sections (`.text`, `.data`, `.debug`, etc.).
- **Top Contributors**: Identify which crates or modules are consuming the most space. Supports deep drill-down with `--depth`.
- **Interactive Explorer (TUI)**: Navigate your binary's size distribution interactively in the terminal.
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

### 2. Interactive Explorer
Launch the TUI to explore size distribution:
```bash
binsize explore target/release/my_app
```

### 3. Find Top Contributors
Identify the largest crates and symbols. Use `--depth` to see sub-modules:
```bash
# Crate level
binsize top target/release/my_app --depth 1

# Module level (drill-down)
binsize top target/release/my_app --depth 3
```

### 4. Compare Binaries (Diff)
See what changed between two versions of a binary:
```bash
binsize diff old_binary new_binary
```

## ⚖️ License
MIT
