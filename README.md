# 📊 binsight

Point **binsight** at any binary. In seconds, know exactly what's large, why it's large, and what to do about it.

**binsight** provides actionable binary health diagnostics to prune bloat.

---

## 📖 Table of Contents
- [Why Binsight?](#-why-binsight)
- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Command Reference](#-command-reference)
- [Output Examples](#-output-examples)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🤔 Why Binsight?

Most binary analysis tools either provide too little information (just total size) or overwhelming low-level dumps. **binsight** bridges this gap by focusing on:
- **Clarity**: Symbols are grouped by crate and module paths.
- **Actionability**: A built-in diagnostics engine identifies common bloat patterns (e.g., excessive monomorphization).
- **Regression Tracking**: A first-class `diff` mode to see exactly how a PR affects binary size.

---

## ✨ Features

- **🔍 Comprehensive Analysis**: Breakdown by sections (`.text`, `.rodata`, `.debug`).
- **📦 Dependency Attribution**: Map code size directly to crates and modules.
- **🩺 Health Diagnostics**: Detects unstripped symbols, generic bloat, panic machinery overhead, and extremely large individual functions.
- **🪟 Windows Support**: Platform-agnostic analysis with placeholder for PDB debug information.
- **📍 Source Mapping**: Link the largest symbols directly to source files and line numbers.
- **⚖️ Diff Mode**: Compare two binaries to see deltas in size and symbol composition.
- **🖥️ Interactive Explorer**: A terminal UI to navigate your binary's hierarchy.

---

## 🚀 Installation

### From Source
Ensure you have the latest stable version of Rust installed:

```bash
git clone https://github.com/sumant1122/binsight.git
cd binsight
cargo install --path .
```

---

## ⚡ Quick Start

Analyze your current project's binary:
```bash
binsight analyze ./target/release/my_app
```

Drill down into specific module bloat:
```bash
binsight top ./target/release/my_app --depth 2
```

---

## 🛠 Command Reference

| Command | Description |
| :--- | :--- |
| `analyze <path>` | High-level breakdown + basic health diagnostics. |
| `diagnose <path>` | Detailed report on monomorphization, panic bloat, etc. |
| `top <path>` | Hierarchical list of contributors (use `--depth`). |
| `explore <path>` | Interactive TUI for binary navigation. |
| `diff <old> <new>` | Compare two binaries and show size deltas. |

---

## 💡 Output Examples

### Hierarchical Attribution (with Source Mapping)
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

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on bug reports, feature requests, and pull requests.

---

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.
