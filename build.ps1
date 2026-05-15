$ErrorActionPreference = "Stop"

Set-Location $PSScriptRoot

function Assert-CleanGitTree {
    param([string]$Name)

    git diff --quiet

    if ($LASTEXITCODE -ne 0) {
        Write-Host "$Name working tree not clean"
        exit 1
    }
}

function Assert-Success {
    param([string]$Message)

    if ($LASTEXITCODE -ne 0) {
        Write-Host $Message
        exit 1
    }
}

Assert-CleanGitTree "XSC"

git switch main
Assert-Success "Failed to switch main"

$p1 = Start-Process cargo `
    -ArgumentList "build --release" `
    -NoNewWindow `
    -PassThru

$p2 = Start-Process cargo `
    -ArgumentList "install --path ./xsc-cli" `
    -NoNewWindow `
    -PassThru

$wslPath = wsl wslpath (Get-Location).Path

$p3 = Start-Process wsl `
    -ArgumentList "bash -lc `"cd '$wslPath' && cargo build --release`"" `
    -NoNewWindow `
    -PassThru

Wait-Process $p1, $p2, $p3

if (
    $p1.ExitCode -ne 0 -or
    $p2.ExitCode -ne 0 -or
    $p3.ExitCode -ne 0
) {
    Write-Host "One or more builds failed"
    exit 1
}

python bumpver.py
Assert-Success "bumpver.py failed"

git add .
git commit -m "Bumpver"
Assert-Success "Git commit failed"

$tag = Read-Host "XSC Enter release tag (e.g. v1.2.3-alpha)"
if ([string]::IsNullOrWhiteSpace($tag)) {
    Write-Host "No tag provided, aborting"
    exit 1
}

if (git tag -l $tag) {
    Write-Host "Tag already exists"
    exit 1
}

git tag $tag

git push
Assert-Success "Git push failed"
git push origin $tag
Assert-Success "Git tag push failed"

$vsceServer = Resolve-Path "../../My Stuff/web dev/VSCE/aoe2xsscripting/server"

Copy-Item `
    "./target/release/xs-check-lsp" `
    "$vsceServer/" `
    -Force

Copy-Item `
    "./target/release/xs-check-lsp.exe" `
    "$vsceServer/" `
    -Force

Set-Location "../../My Stuff/web dev/VSCE/aoe2xsscripting"

Assert-CleanGitTree "VSC"

git switch dev-lsp
Assert-Success "Failed to switch dev-lsp"

# Prevent npm from auto committing/tagging
npm version prerelease --preid=alpha --no-git-tag-version
Assert-Success "npm version failed"

vsce package
Assert-Success "vsce package failed"

git add .
git commit -m "Bumpver"
Assert-Success "VSC commit failed"

$tag = Read-Host "VSC Enter release tag (e.g. v1.2.3-alpha)"
if ([string]::IsNullOrWhiteSpace($tag)) {
    Write-Host "No tag provided, aborting"
    exit 1
}

if (git tag -l $tag) {
    Write-Host "Tag already exists"
    exit 1
}

git tag $tag

git push
Assert-Success "VSC push failed"

git push origin $tag
Assert-Success "VSC push failed"

Write-Host ""
Write-Host "Build pipeline completed successfully"