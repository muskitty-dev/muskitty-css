#!/usr/bin/env bash
# 在独立 CI runner 上克隆 path 依赖到预期的 ../ 相对位置。
# muskitty-css 依赖 muskitty-css-tokenizer + muskitty-css-parser。
# 幂等：若目录已存在则跳过。

set -euo pipefail

clone_if_absent() {
  local url="$1"
  local dest="$2"
  if [ ! -d "$dest" ]; then
    git clone --depth 1 "$url" "$dest"
  else
    echo "$dest already exists, skipping clone."
  fi
}

clone_if_absent https://github.com/muskitty-dev/muskitty-css-tokenizer.git ../muskitty-css-tokenizer
clone_if_absent https://github.com/muskitty-dev/muskitty-css-parser.git ../muskitty-css-parser
