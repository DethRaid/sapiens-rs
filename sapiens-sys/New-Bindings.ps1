mkdir extern
cd extern
git clone https://github.com/mjdave/sapiens-mod-creation.git
cd ..
bindgen --impl-partialeq --impl-debug --with-derive-default --with-derive-eq -o src/bindings.rs resources/wrapper.h -- -Iextern/sapiens-mod-creation/SPlugins/include/
Remove-Item -Recurse -Force extern

# Update version
$ManifestFileName = "Cargo.toml"
$ManifestFile = Get-Content $ManifestFileName
$ManifestFileVersion = ($ManifestFile | Select -Index 2).Split('.')
$NewVersion = "{0}.{1}.{2}" -f $ManifestFileVersion[0], ([int]$ManifestFileVersion[1] + 1), $ManifestFileVersion[2]
$ManifestFile -Replace "version = .\d+.\d+.\d+.", $NewVersion | Set-Content $ManifestFileName