param(
    [Parameter(Mandatory = $true)]
    [string]$InputApk,
    [string]$OutputDir = (Join-Path $PSScriptRoot '..\dist'),
    [string]$Rustc = ((Get-Command rustc -ErrorAction Stop).Source),
    [string]$Python = ((Get-Command python -ErrorAction Stop).Source)
)

$ErrorActionPreference = 'Stop'
& $Python (Join-Path $PSScriptRoot 'build_router.py') $InputApk --output-dir $OutputDir --rustc $Rustc
exit $LASTEXITCODE
