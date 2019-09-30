if (${args}.Length -ne 1) {
    Write-Output "Error: expected argument is 1, but actual is $(${args}.Length)."
    exit 1
}

$base_path = Split-Path $MyInvocation.MyCommand.Path
$bin_dir = "${base_path}\src\bin"
$target_dir = "${base_path}\target\debug"
$p_name = ${args}[0]
$test_dir = "${target_dir}\${p_name}-test"

cargo build --bin "${p_name}"

if (!(Test-Path ${test_dir})) {
    $url = (Read-Host "Info: input problem url. > ")
    New-Item ${test_dir} -ItemType Directory -Force
    cd ${test_dir}
    oj dl ${url}
}

cd ${test_dir}
oj test -c "..\\${p_name}.exe"
cd ${base_path}
