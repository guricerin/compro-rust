#!/bin/bash

# 引数で指定した問題の回答をビルドし、テストする。

set -eu

if [ ${#} -ne 1 ]; then
    echo "Error: Expected 1 argument. But real is ${#}."
    exit 1
fi

readonly p_name=${1}

cargo run --bin ${p_name}
