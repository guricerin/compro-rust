#!/bin/bash

set -eu

readonly base_path=$(cd $(dirname ${0}); pwd)
readonly bin_dir="${base_path}/src/bin"
readonly temp_dir="${base_path}/template"

if [ -d ${bin_dir} ]; then
    read -p "Info: ${bin_dir} is already exists. Remake that? [y/n] > " res
    if [ ${res} != "y" ]; then
        echo "Info: stop process."
        exit 1
    fi
fi

mkdir -p ${bin_dir}

for p_name in {a..f}; do
    cp ${temp_dir}/main.rs.temp ${bin_dir}/${p_name}.rs
done

# online-judge-tools が生成した問題毎のtestディレクトリをすべて削除。
readonly target_dir="${base_path}/target/debug"

for p_name in {a..f}; do
    test_dir="${target_dir}/${p_name}-test"
    rm -rf ${test_dir}
done

echo "Making src/bin dir have done. Go For It!"
