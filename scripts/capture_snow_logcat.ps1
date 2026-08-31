param(
    [int]$Seconds = 12,
    [string]$Package = 'com.miui.weather3',
    [string]$Component = 'com.miui.weather3/android.app.NativeActivity'
)

$ErrorActionPreference = 'Stop'
$Root = Split-Path -Parent $PSScriptRoot
$Adb = Join-Path $env:LOCALAPPDATA 'Android\Sdk\platform-tools\adb.exe'
$OutputDir = Join-Path $Root 'analysis\snow_router_test'
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
$devices = @(& $Adb devices | Select-String "\tdevice$")
if ($devices.Count -ne 1) { throw "Expected one ADB device, found $($devices.Count)" }

& $Adb shell am force-stop $Package
Start-Sleep -Seconds 3
& $Adb logcat -c -b main -b system -b crash
$stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$log = Join-Path $OutputDir ("weather3_snow_realtime_" + $stamp + ".log")
$stderr = Join-Path $OutputDir ("weather3_snow_realtime_" + $stamp + "_stderr.txt")
$stream = Start-Process -FilePath $Adb -ArgumentList @(
    'logcat', '-b', 'main', '-b', 'system', '-b', 'crash', '-v', 'threadtime',
    'SnowWeatherRouter:V', 'flutter:V', 'DartVM:V', '*:S'
) -RedirectStandardOutput $log -RedirectStandardError $stderr -PassThru -NoNewWindow
Start-Sleep -Milliseconds 400
& $Adb shell am start -W -n $Component
Start-Sleep -Seconds $Seconds
if (-not $stream.HasExited) { Stop-Process -Id $stream.Id -Force }
$stream.WaitForExit()

Write-Output "LOG=$log"
Get-Item $log | Select-Object Length, FullName | Format-List
Get-Content -LiteralPath $log -Raw
