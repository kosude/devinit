#!/usr/bin/env bash

GIT="$(command -v git)"

OUT=
SHORT=
NO_COMMITN=

for arg in "$@"; do
    case "$arg" in
        --long)
            OUT="$($GIT describe --abbrev=4 --always --tags --dirty 2>/dev/null)"
            ;;
        --short)
            OUT="$($GIT describe --abbrev=4 --always --tags --dirty 2>/dev/null)"
            OUT="${OUT%-g*}"
            if [ "$?" != 0 ]; then
                OUT="v0.0.0"
            fi
            SHORT=true
            ;;
        --no-commitn)
            NO_COMMITN=true
            ;;
    esac
done

if [ ! -n "$OUT" ]; then
    OUT="$(getverslong)"
fi

if [ -n "$NO_COMMITN" ] && [ -n "$SHORT" ]; then
    OUT="${OUT%-*}"
fi

echo $OUT
