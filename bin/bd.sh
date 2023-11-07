#!/usr/bin/env bash
# bd = build & deploy

cargo build && rm -rf ../rust-reinze/plugins/libreinze_lib_util_*.so && cp target/debug/libreinze_lib_util.so ../rust-reinze/plugins/libreinze_lib_util_$(date "+%Y%m%dT%H%M%S").so
