set dotenv-load := true

default:
    @just --list

alias run := start
alias dev := start
start:
    cargo run -p unwrapped
