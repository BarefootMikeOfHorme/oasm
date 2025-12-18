# OASM Module Usage

This module integrates OASM into PowerShell and CMD shells.

## Cmdlets
- Invoke-OasmCompile — Compile OASM source into IR.
- Invoke-OasmLink — Link IR into an executable.
- Start-Oasm — Run an OASM executable.
- Get-OasmLog — Retrieve runtime logs.

## Quick Example
Invoke-OasmCompile -Source .\src\demo.oasm -Out .\build\demo.ir
Invoke-OasmLink -Input .\build\demo.ir -Out .\build\demo.exe
Start-Oasm -Exe .\build\demo.exe
Get-OasmLog
