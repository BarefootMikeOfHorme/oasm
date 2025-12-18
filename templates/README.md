# OASM Template Library

**Executive Function AI + CAD Automation Templates**

## Overview

The template library provides reusable, high-level patterns for:
- **Schemas**: SmartObject containers, validation rules, manifests
- **Scripts**: PowerShell, Python, Rust code generation
- **Commands**: Diagnostic blocks, health checks, automation
- **Workflows**: CAD design flows, executive function pathways
- **Scans**: Deep codebase analysis, metrics collection

## Directory Structure

```
templates/
├── schemas/          # Data structure templates
│   └── smartobject_container.yaml
├── scripts/          # Code generation templates
│   └── script_template_generator.yaml
├── commands/         # Command block templates
│   └── project_health_check.yaml
├── workflows/        # Workflow presets
│   └── cad_automation_preset.yaml
└── scans/            # Analysis templates
    └── codebase_deep_scan.yaml
```

## Usage

### 1. Schema Templates
Create SmartObject containers for Objex CAD:
```bash
oasm template apply schemas/smartobject_container.yaml --object gear_assembly
```

### 2. Scan Templates
Run deep codebase analysis:
```bash
oasm scan apply scans/codebase_deep_scan.yaml --output logs/analysis/
```

### 3. Workflow Templates
Execute CAD automation preset:
```bash
oasm workflow start workflows/cad_automation_preset.yaml
```

### 4. Command Blocks
Quick project health check:
```bash
oasm command run commands/project_health_check.yaml
```

### 5. Script Generation
Generate PowerShell script from template:
```bash
oasm script generate --template scripts/script_template_generator.yaml \
                      --language powershell \
                      --name my_automation
```

## Executive Function Features

All templates support:
- ✅ **Progress Tracking**: Visual indicators and completion percentages
- ✅ **Task Breakdown**: Clear phases with discrete steps
- ✅ **Error Recovery**: Helpful suggestions when things fail
- ✅ **Checkpoints**: Auto-save and resume capability
- ✅ **Reminders**: Last action, next steps, time estimates

## Integration with wpshell / PS 2026

Templates are designed to work with PowerShell Insider 2026 features:
- AI-assisted command completion
- Autonomous workflow execution
- Smart error handling
- Context-aware suggestions

## Creating Custom Templates

See `docs/template_creation_guide.md` for details on:
- Template schema specification
- Placeholder syntax
- Validation rules
- Integration points

## Template Categories

### Preset (System-Defined)
Ready-to-use templates in this directory.

### User-Defined
Store in `~/.oasm/templates/` for personal customization.

## Examples

### SmartObject Creation
```yaml
# From smartobject_container.yaml
oasm create gear --from-template schemas/smartobject_container \
                 --param teeth=20 \
                 --param module=2.5
```

### Full CAD Workflow
```yaml
# From cad_automation_preset.yaml
oasm workflow start cad \
    --primitive cylinder \
    --params "diameter=50,height=100" \
    --export step,stl
```

### Metrics Scan
```yaml
# From codebase_deep_scan.yaml
oasm scan deep --format json --output dashboard/
```
