#!/bin/bash

# Require: https://github.com/kmky/online-judge-tools

# 引数で指定した問題の回答をビルドし、テストする。

set -eu

if [ ${#} -ne 1 ]; then
    echo "Error: Expected 1 argument. But actual is ${#}."
    exit 1
fi

readonly base_path=$(cd $(dirname ${0}); pwd)
readonly bin_dir="${base_path}/src/bin"
readonly target_dir="${base_path}/target/debug"
readonly p_name=${1}
readonly test_dir="${target_dir}/${p_name}-test"

cargo build --bin ${p_name}

if [ ! -d ${test_dir} ]; then
    read -p "Info: input problem url. > " url
    mkdir -p ${test_dir}
    cd ${test_dir}
    oj dl ${url}
fi

cd ${test_dir}
oj test -c ${target_dir}/${p_name}
