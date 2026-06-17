# Zed (q3ik-edit fork)

[![Zed Fork](https://img.shields.io/badge/Fork-xjayk%2Fq3ik--edit-blue)](https://github.com/xjayk/q3ik-edit)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](LICENSE-GPL)

Welcome to **Zed (q3ik-edit)**, a fork of the high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

> [!IMPORTANT]
> **Repository Boundary Rule**  
> This project operates strictly under a repository boundary.
> - **Allowed Target**: All commits, pushes, and Pull Requests MUST be targetted exclusively to this fork repository: `https://github.com/xjayk/q3ik-edit`.
> - **Forbidden Target**: Under no circumstances should any modifications or pull requests be pushed or opened against the upstream repository `https://github.com/zed-industries/zed`.

---

## Workspace Customizations

This fork features a modified workspace optimized for a leaner codebase footprint:
- **No REPL / Notebook Support**: The `repl` crate has been completely removed.
- **No Debugger / DAP Support**: The debugger UI (`debugger_ui`), debugger tools (`debugger_tools`), DAP client/adapters (`dap`, `dap_adapters`), and related settings/menu options have been removed.

---

## Developing

### Platform-Specific Guides
- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

---

## Contributing

Please review the contribution guidelines in [CONTRIBUTING.md](./CONTRIBUTING.md) (or checkout the rules in [.rules](./.rules)) for coding standards, pull request hygiene, and GPUI development practices.

All agent-driven edits must comply with the repository boundary rules described above.

---

## Licensing

Zed source code is licensed primarily under the GPL-3.0-or-later license, with Apache-2.0 components where marked.
For details on open-source dependencies and license compliance, see the main [LICENSE-GPL](LICENSE-GPL) and [LICENSE-APACHE](LICENSE-APACHE) files.
