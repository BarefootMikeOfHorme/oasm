# OASM Manifest System

**Central Registry for Easy Location and Loading of Modules, Files, and Settings**

## Overview

The manifest system provides a **single source of truth** for all OASM components:
- Module locations and entry points
- Configuration files and schemas
- Template library index
- Output directories
- Integration points (wpshell, Objex, Python, PowerShell)
- Serialization formats (CBOR, YAML, JSON, HDF5)

## Master Manifest

Location: `manifests/master_manifest.yaml`

### Structure

```yaml
serialization:     # Format specifications (OASM vs Objex)
modules:           # All executable modules (daemon, shell, UI, etc.)
configs:           # Configuration file locations
schemas:           # Schema registry
templates:         # Template library index
outputs:           # Log and export directories
integrations:      # External system integration points
capabilities:      # Available security capabilities
load_order:        # Bootstrap, startup, on-demand phases
health:            # Health monitoring configuration
```

## Usage

### PowerShell (Easy Access)

```powershell
# List all modules
.\scripts\PS\Get-OasmManifest.ps1 -Type module -List

# Get specific module
.\scripts\PS\Get-OasmManifest.ps1 -Type module -Name runtime_daemon

# List configs
.\scripts\PS\Get-OasmManifest.ps1 -Type config -List

# Get schema
.\scripts\PS\Get-OasmManifest.ps1 -Type schema -Name oasm_protocol

# View all integrations
.\scripts\PS\Get-OasmManifest.ps1 -Type integration

# Show everything
.\scripts\PS\Get-OasmManifest.ps1 -Type all
```

### Rust (Programmatic Access)

```rust
use runtime_daemon::manifest_loader::ManifestLoader;

// Load manifest
let loader = ManifestLoader::load("manifests/master_manifest.yaml")?;

// Get module path
if let Some(path) = loader.module_path("runtime_daemon") {
    println!("Daemon at: {}", path.display());
}

// Get module entry point
if let Some(exe) = loader.module_entry("oasm_shell") {
    println!("Shell executable: {}", exe.display());
}

// Get config file
if let Some(config) = loader.config_path("runtime") {
    println!("Runtime config: {}", config.display());
}

// Get schema
if let Some(schema) = loader.schema_path("oasm_protocol") {
    println!("Protocol schema: {}", schema.display());
}

// Get template
if let Some(template) = loader.template_path("scans", "codebase_deep_scan.yaml") {
    println!("Scan template: {}", template.display());
}

// Check capability
if loader.has_capability("file_watch") {
    println!("File watching is available");
}

// Get auto-start modules
for module in loader.auto_start_modules() {
    println!("Auto-start: {}", module.name);
}
```

## Module Registry

All modules are registered with:
- **ID**: Unique identifier
- **Name**: Human-readable name
- **Type**: service, tool, shell, ui, library
- **Location**: Directory path
- **Entry**: Executable path (if applicable)
- **Config**: Configuration file
- **Capabilities**: What the module can do
- **Auto-start**: Whether to start automatically
- **Dependencies**: Required modules

### Registered Modules

1. **runtime_daemon** - Supervisor loop (auto-starts)
2. **compiler** - OASM compilation
3. **scanner** - Pre-compile diagnostics
4. **oasm_shell** - Interactive CLI
5. **rust_ui** - UI interface
6. **bindings** - Native DLL bindings

## Configuration Registry

Configs are organized by subsystem:
- **runtime**: Main OASM runtime
- **ui**: UI settings
- **daemon**: Supervisor configuration
- **shell**: Shell profile
- **compiler**: Compilation settings

Each config has:
- **primary**: Main config file
- **schema**: Validation schema
- **fallback**: Default config

## Schema Registry

Schemas define validation rules for:
- **oasm_config**: Runtime configuration
- **oasm_protocol**: CBOR event protocol
- **cli_state**: Compilation status
- **diagnostic_index**: Error tracking
- **bindings_manifest**: DLL loading
- **smartobject_container**: Objex containers
- **scan_template**: Scan configurations
- **workflow_template**: Workflow definitions

## Template Library

Organized into categories:
- **schemas**: Data structure templates
- **scripts**: Code generation templates
- **commands**: Command block templates
- **workflows**: Workflow presets
- **scans**: Analysis templates

## Serialization Formats

### OASM
- **Primary**: CBOR (canonical runtime state)
- **Mirror**: YAML (human-readable with comments)
- **Logs**: JSON (structured logs and indexes)
- **Schemas**: JSON Schema

### Objex
- **Archive**: HDF5 (long-term storage)
- **Runtime**: CBOR (live SmartObject state)
- **Exports**: STEP, IGES, STL, DXF, SVG
- **Metadata**: YAML (export provenance)

## Integration Points

### PowerShell 2026
- Module: `shells/psmodule`
- Scripts: `scripts/PS`
- Entry: `Invoke-OasmScan.ps1`

### Python 3.12
- Plugins: `ui/python_ui_plugins/plugins`
- Venv: `environments/coding`

### wpshell (Custom PowerShell)
- Enabled: `true`
- Profile: `default`

### Objex CAD
- Enabled: `true`
- HDF5 Archives: `../Objex/archives`
- Primitives: `../Objex/kernel/primitives`

## Load Order

### Bootstrap Phase
1. Load all config files
2. Validate against schemas
3. Load native DLL bindings

### Startup Phase
1. Start runtime daemon (supervisor)
2. Index template library

### On-Demand Phase
- Compiler
- Scanner
- OASM Shell
- UI

## Health Monitoring

Tracked files:
- `heartbeat.json` - Daemon alive indicator
- `daemon_status.json` - Supervisor state
- `context_status.json` - Token usage

Health checks:
- **runtime_daemon**: Every 5 seconds
- **bindings**: On startup

Alerts:
- Heartbeat timeout: 5 minutes
- Daemon inactive: 10 minutes

## Benefits

**For Developers:**
- Single source of truth for all paths
- Easy module discovery
- Clear dependency tracking
- Automated loading order

**For Executive Function:**
- Quick location of settings
- Template discovery
- Output path clarity
- Integration visibility

**For System Integration:**
- Clear serialization strategy
- Module capability tracking
- Health monitoring config
- Cross-system integration points

## Example Workflows

### Find and Run Scanner
```powershell
# Find scanner executable
$scanner = .\scripts\PS\Get-OasmManifest.ps1 -Type module -Name scanner

# Run it
& $scanner.entry --help
```

### Load Runtime Config
```rust
let loader = ManifestLoader::load("manifests/master_manifest.yaml")?;
let config_path = loader.config_path("runtime").unwrap();
let config = std::fs::read_to_string(config_path)?;
```

### Check Objex Integration
```powershell
$manifest = Get-Content manifests\master_manifest.yaml | ConvertFrom-Yaml
if ($manifest.integrations.objex.enabled) {
    Write-Host "Objex integration enabled"
    Write-Host "HDF5 archives: $($manifest.integrations.objex.hdf5_archives)"
}
```

---

**The manifest system makes OASM components easily discoverable and loadable for both human users and automated systems.**
