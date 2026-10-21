<#
.SYNOPSIS
    取知意（ONNX）要的运行库到 target\installer\onnxruntime\：onnxruntime.dll、onnxruntime_providers_shared.dll、DirectML.dll 与许可。
.DESCRIPTION
    ONNX Runtime 1.24.4 DirectML 版（DirectML 版到 1.24.4 为止）取自 NuGet 的 Microsoft.ML.OnnxRuntime.DirectML，
    DirectML.dll 1.15.4（1.24.4 要的版本）取自 NuGet 的 Microsoft.AI.DirectML，许可与第三方声明一起取。两个包都按 SHA-256 核对，已取过就跳过。
    DirectML 那个包有 200 MB（各种平台都在里面），只取一次；PyPI 的轮子小得多，但国内常连不上。
    Server 开发时也从这里加载（见 apps\windows\server\src\dispatch\rescore\models.rs），装机时 qingjian.iss 把它们装到 exe 旁边。
#>
$ErrorActionPreference = 'Stop'
# 进度条会让 Invoke-WebRequest 慢好几倍
$ProgressPreference = 'SilentlyContinue'

$Version = '1.24.4'
$Packages = @(
    @{
        Url    = "https://api.nuget.org/v3-flatcontainer/microsoft.ml.onnxruntime.directml/$Version/microsoft.ml.onnxruntime.directml.$Version.nupkg"
        Sha256 = '57e9f11b73437bef7a309496135d4c1f96b1a8e9ddba60013fa27bfc1d788681'
        Files  = @{
            'runtimes/win-x64/native/onnxruntime.dll'                  = 'onnxruntime.dll'
            'runtimes/win-x64/native/onnxruntime_providers_shared.dll' = 'onnxruntime_providers_shared.dll'
            'LICENSE'                                                  = 'onnxruntime-LICENSE.txt'
            'ThirdPartyNotices.txt'                                    = 'onnxruntime-ThirdPartyNotices.txt'
        }
    },
    @{
        Url    = 'https://api.nuget.org/v3-flatcontainer/microsoft.ai.directml/1.15.4/microsoft.ai.directml.1.15.4.nupkg'
        Sha256 = '4e7cb7ddce8cf837a7a75dc029209b520ca0101470fcdf275c1f49736a3615b9'
        Files  = @{
            'bin/x64-win/DirectML.dll' = 'DirectML.dll'
            'LICENSE.txt'              = 'DirectML-LICENSE.txt'
            'ThirdPartyNotices.txt'    = 'DirectML-ThirdPartyNotices.txt'
        }
    }
)

$Repo  = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
$Dest  = Join-Path $Repo 'target\installer\onnxruntime'
$Stamp = Join-Path $Dest 'VERSION'
if ((Test-Path $Stamp) -and ((Get-Content $Stamp -Raw).Trim() -eq $Version)) {
    Write-Host "ONNX Runtime $Version 已在 $Dest" -ForegroundColor Cyan
    return
}
if (Test-Path $Dest) { Remove-Item $Dest -Recurse -Force }
New-Item -ItemType Directory -Path $Dest -Force | Out-Null

Add-Type -AssemblyName System.IO.Compression.FileSystem
foreach ($package in $Packages) {
    $download = Join-Path $env:TEMP ([IO.Path]::GetFileName($package.Url))
    Write-Host "下载 $($package.Url)" -ForegroundColor Cyan
    Invoke-WebRequest -Uri $package.Url -OutFile $download -UseBasicParsing
    $hash = (Get-FileHash $download -Algorithm SHA256).Hash.ToLower()
    if ($hash -ne $package.Sha256) {
        Remove-Item $download -Force
        throw "$download 的 SHA-256 是 $hash，应为 $($package.Sha256)"
    }
    $zip = [IO.Compression.ZipFile]::OpenRead($download)
    try {
        foreach ($entry in $package.Files.Keys) {
            $found = $zip.Entries | Where-Object { $_.FullName -eq $entry } | Select-Object -First 1
            if (-not $found) { throw "$download 里没有 $entry" }
            [IO.Compression.ZipFileExtensions]::ExtractToFile($found, (Join-Path $Dest $package.Files[$entry]), $true)
        }
    } finally {
        $zip.Dispose()
        Remove-Item $download -Force
    }
}
Set-Content -Path $Stamp -Value $Version -Encoding ASCII
Write-Host "ONNX Runtime $Version（DirectML）→ $Dest" -ForegroundColor Green
