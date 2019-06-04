#!/usr/bin/env bash

script="$1"
gif="$(sed -ne 's/basename \(.*\)$/\1/p' $1)"

if [[ "$gif" != "" ]]; then
    gif="out/${gif}.gif"
    animate "$gif"
fi
