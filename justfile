#!/usr/bin/env -S just --justfile

default:
    just --list

cargo *args:
    cargo +nightly -Z unstable-options -C rust {{ args }}

[group('nds')]
check-nds *args:
    cargo +nightly -Z unstable-options -C rust check --target armv5te-none-eabi {{ args }}

[group('nds')]
clippy-nds *args:
    cargo +nightly -Z unstable-options -C rust clippy --target armv5te-none-eabi {{ args }}

[group('nds')]
build-nds game profile *args:
    cargo +nightly -Z unstable-options -C rust build --target armv5te-none-eabi --profile {{ profile }}
    armips -stat -strequ PROFILE {{ if profile == "dev" { "debug" } else { profile } }} {{ args }} {{ game }}.asm
    mnl-nds-pack -o {{ game }}.nds -d {{ game }}-data
