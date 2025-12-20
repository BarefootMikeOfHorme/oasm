# OASM: Objex AsSeMbly Module

<div align="center">

![OASM Logo](https://img.shields.io/badge/OASM-Objex%20AsSeMbly-orange?style=for-the-badge&logo=rust)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![PowerShell](https://img.shields.io/badge/powershell-7.0+-blue?style=flat-square&logo=powershell)](https://docs.microsoft.com/en-us/powershell/)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

**Assembly-like enhancement for any program. Like NASM/MASM, but for high-level operations.**

[Features](#features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Structure](#project-structure)

</div>

---

## Features

OASM provides **low-level control** over complex software systems using an assembly-inspired instruction set.

* ✨ **Universal Assembly** - Standardized instructions (`CREATE`, `SET`, `EXTRUDE`, `VALIDATE`) for diverse domains.
* 📐 **CAD Integration** - First-class support for **OpenCASCADE (OCCT 7.9.3)**.
* 🛡️ **Type-Safe Core** - Built in Rust for maximum performance and reliability.
* 🐚 **Native Shells** - Deep integration with PowerShell for ease of use.
* 🧩 **Modular Rules** - Extensible validation and constraint system.

---

## Quick Start

### PowerShell Usage

```powershell
# Import OASM module
Import-Module .\shells\psmodule\OASM.PowerShell.psd1

# Initialize and create a CAD context
Initialize-Oasm
$ctx = New-OasmContext -ProgramType CAD

# Execute OASM Assembly
$source = @'
CREATE gear
  SET teeth = 20
  SET module = 2.5
EXTRUDE z_axis, 10
VALIDATE topology
EXPORT step, "output/gear.step"
'@

Invoke-OasmAssembly -Source $source -Context $ctx
```

---

## Project Structure

Verified and organized for clarity:

```text
oasm/
├── bindings/       # Language/runtime bindings (Rust ↔ C, Python, FFI)
├── compiler/       # OASM compiler core logic
├── crates/         # Sub-crates (oasm-api, oasm-core, etc.)
├── docs/           # Documentation and design notes (governance, vision, etc.)
├── examples/       # OASM assembly source examples
├── manifests/      # YAML configuration and cratemanifests
├── native_libs/    # Shared libraries (DLLs)
├── runtime/        # Runtime artifacts and supervisor daemon
├── schemas/        # Schema definitions (YAML/JSON)
├── scripts/        # PowerShell and automation scripts
├── shells/         # Shell integrations (e.g., PowerShell module)
├── ui/             # Rust-based UI implementation
└── logs/           # Organized system and compilation logs
```

---

## Development

### Building the Project

```bash
# Build the entire workspace
cargo build --workspace

# Build the scanner specifically
cargo build --bin oasm-scan
```

### Core Cmdlets

| Cmdlet | Description |
| :--- | :--- |
| `Initialize-Oasm` | Initialize the environment |
| `New-OasmContext` | Create a new program context (e.g., CAD) |
| `Invoke-OasmAssembly` | Run OASM assembly source |
| `Invoke-OasmScan` | Run the codebase diagnostics scanner |

---

<div align="center">
<b>OASM: Powering the next generation of modular automation.</b>
</div>
