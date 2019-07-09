$base_path = Split-Path $MyInvocation.MyCommand.Path
$bin_dir = "${base_path}\src\bin"
$temp_dir = "${base_path}\template"

if (Test-Path ${bin_dir}) {
    $res = (Read-Host "Info: ${bin_dir} is already exists. Remake that? [y/n] > ")
    if (${res} -ne "y") {
        Write-Output "Info: stop process."
        exit 1
    }
}

New-Item ${bin_dir} -ItemType Directory -Force

$p_names = @("a","b","c","d","e","f") 

${p_names} | % {Copy-Item ${temp_dir}\main.rs.temp ${bin_dir}\${_}.rs}

# online-judge-tools が生成した問題毎のtestディレクトリをすべて削除。
$target_dir = "${base_path}\target\debug"
${p_names} | % {Remove-Item ${target_dir}\${_}-test -Recurse}

Write-Output "Making src\bin dir have done. Go For It!"