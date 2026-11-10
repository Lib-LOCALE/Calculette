# Regenerate Flatpak's offline registry sources from Cargo.lock
$projectRoot = Split-Path -Parent $PSScriptRoot
$lines = Get-Content "$projectRoot\Cargo.lock"
$sources = [System.Collections.Generic.List[object]]::new()

$current = @{}
$inPackage = $false

function Process-Package($p) {
    if ($p.source -eq "registry+https://github.com/rust-lang/crates.io-index" -and $p.name -and $p.version -and $p.checksum) {
        $dest = "cargo/vendor/$($p.name)-$($p.version)"
        $sources.Add([ordered]@{
            type = "archive"
            "archive-type" = "tar-gzip"
            url = "https://static.crates.io/crates/$($p.name)/$($p.name)-$($p.version).crate"
            sha256 = $p.checksum
            dest = $dest
        })
        $sources.Add([ordered]@{
            type = "inline"
            contents = "{`"package`": `"$($p.checksum)`", `"files`": {}}"
            dest = $dest
            "dest-filename" = ".cargo-checksum.json"
        })
    }
}

foreach ($line in $lines) {
    if ($line -match '^\[\[package\]\]') {
        if ($inPackage) {
            Process-Package $current
        }
        $current = @{}
        $inPackage = $true
    } elseif ($inPackage) {
        if ($line -match '^name\s*=\s*"([^"]+)"') { $current.name = $matches[1] }
        elseif ($line -match '^version\s*=\s*"([^"]+)"') { $current.version = $matches[1] }
        elseif ($line -match '^source\s*=\s*"([^"]+)"') { $current.source = $matches[1] }
        elseif ($line -match '^checksum\s*=\s*"([^"]+)"') { $current.checksum = $matches[1] }
    }
}
if ($inPackage) {
    Process-Package $current
}

$sources.Add([ordered]@{
    type = "inline"
    contents = "[source.crates-io]`nreplace-with = `"vendored-sources`"`n[source.vendored-sources]`ndirectory = `"cargo/vendor`"`n"
    dest = "cargo"
    "dest-filename" = "config.toml"
})

$json = ConvertTo-Json -InputObject $sources -Depth 10
[System.IO.File]::WriteAllText("$projectRoot\flatpak\cargo-sources.json", ($json + "`n"), [System.Text.UTF8Encoding]::new($false))
Write-Host "$([int]($sources.Count / 2)) crates locked for offline Flatpak builds"
