# q3ik Edit (workspace-free Zed edition)
[![Zed Fork](https://img.shields.io/badge/Fork-zed--industries%2Fzed-blue)](https://github.com/zed-industries/zed)

A lightweight, workspace-free fork of [Zed Industries' Zed](https://github.com/zed-industries/zed). This version is designed to eliminate project clutter by enforcing a single global session, supporting standalone tabs, and providing instant hot-exit capabilities.


## Key Enhancements

Unlike standard Zed, which revolves around project workspaces and folders, this build treats files as independent, lightweight entities.

* **Workspace-Free Architecture:** No project sidebars, folder indexing, or multi-file project clutter.
* **Single Global Session:** Open files instantly from anywhere in your system into one unified instance.
* **Standalone Tabs:** Every file behaves as an independent, floating tab context.
* **Instant Hot-Exit:** Close the application instantly without prompts. Your un-saved changes are cached globally and restored immediately upon relaunch.

## Workspace Customizations

This fork features a modified workspace optimized for a leaner codebase footprint:
- **No REPL / Notebook Support**: The `repl` crate has been completely removed.
- **No Debugger / DAP Support**: The debugger UI (`debugger_ui`), debugger tools (`debugger_tools`), DAP client/adapters (`dap`, `dap_adapters`), and related settings/menu options have been removed.


---

> [!IMPORTANT]
> **Repository Boundary Rule**  
> This repository has been **intentionally detached** from the upstream GitHub fork network. 
> The project operates strictly under a repository boundary.
> - **Allowed Target**: All commits, pushes, and Pull Requests MUST be targetted exclusively to this fork repository: `https://github.com/xjayk/q3ik-edit`.
> - **Forbidden Target**: Under no circumstances should any modifications or pull requests be pushed or opened against the upstream repository `https://github.com/zed-industries/zed`.
>
> All feature development, bug fixes, and internal pull requests must be directed **strictly to this repository**.
---

## Licensing & Credits
This project is a modified derivative of the open-source editor created by **Zed Industries, Inc.** 
In compliance with open-source licensing constraints, this software remains distributed under the original licenses provided by the upstream authors:
* The editor codebase is licensed under the **GNU General Public License v3.0** (`GPL-3.0`).
* Server components are licensed under the **GNU Affero General Public License v3.0** (`AGPL-3.0`).
