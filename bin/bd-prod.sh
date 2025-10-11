#!/usr/bin/env bash
# bd = build & deploy

[[ $(uname -s) == "Darwin" ]] && export extension=.dylib || export extension=.so

cargo update && \
  cargo build --release && \
  rm -rf ../rust-reinze/plugins/libreinze_lib_util_* && \
  mv "target/release/libreinze_lib_util${extension}" "../rust-reinze/plugins/libreinze_lib_util_$(date "+%Y%m%dT%H%M%S")${extension}"
