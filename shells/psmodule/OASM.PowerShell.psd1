@{
    RootModule        = 'OASM.PowerShell.psm1'
    ModuleVersion     = '0.1.0'
    GUID              = '12345678-abcd-1234-abcd-1234567890ab'
    Author            = 'OASM Project'
    Description       = 'OASM assembly module for wpshell - provides assembly-like control for programs'
    PowerShellVersion = '5.1'
    FunctionsToExport = @(
        'Initialize-Oasm',
        'New-OasmContext',
        'Invoke-OasmAssembly',
        'Get-OasmRules',
        'Get-OasmBlocks',
        'Invoke-OasmCompile',
        'Invoke-OasmScan',
        'Start-OasmDaemon'
    )
    CmdletsToExport   = @()
    VariablesToExport = @()
    AliasesToExport   = @()
}
