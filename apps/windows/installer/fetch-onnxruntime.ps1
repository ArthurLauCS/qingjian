<#
.SYNOPSIS
    取知意（ONNX）要的运行库到 target\installer\onnxruntime\：onnxruntime.dll、onnxruntime_providers_shared.dll、DirectML.dll 与许可。
.DESCRIPTION
    ONNX Runtime 1.24.4 DirectML 版（DirectML 版到 1.24.4 为止）取自 NuGet 的 Microsoft.ML.OnnxRuntime.DirectML；
    DirectML.dll（1.24.4 要的 1.15.x）取自 PyPI 的 onnxruntime-directml 轮子，比 NuGet 的 Microsoft.AI.DirectML（200 MB）小得多，
    它的许可与第三方声明在 licenses\ 里（从 Microsoft.AI.DirectML 1.15.4 取的）。两个包都按 SHA-256 核对，已取过就跳过。
    Server 开发时也从这里加载（见 apps\windows\server\src\dispatch\rescore\models.rs），装机时 qingjian.iss 把它们装到 exe 旁边。
#>
$ErrorActionPreference = 'Stop'

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
        Url    = "https://files.pythonhosted.org/packages/88/ea/33814eb0ec96775eda4c1d30b0d86e91d7d2cd0d84c66d3915aef0e06fa3/onnxruntime_directml-$Version-cp312-cp312-win_amd64.whl"
        Sha256 = 'f2ecb68b7b7b259d2ef3112ae760149f9b5a1e7c0fbb73d539da6250a648a614'
        Files  = @{ 'onnxruntime/capi/DirectML.dll' = 'DirectML.dll' }
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
Copy-Item (Join-Path $PSScriptRoot 'licenses\*') -Destination $Dest
Set-Content -Path $Stamp -Value $Version -Encoding ASCII
Write-Host "ONNX Runtime $Version（DirectML）→ $Dest" -ForegroundColor Green
