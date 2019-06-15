#!/usr/bin/env bash

args="-v"
for crate in *graphics* parse_obj; do
    args="${args} -p ${crate}"
done

eval "cargo clean" "$args"
eval "cargo clean --release" "$args"
