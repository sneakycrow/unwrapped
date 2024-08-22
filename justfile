set dotenv-load := true

default:
    @just --list

alias run := start
alias dev := start
start:
    cargo run -p unwrapped

migrate *FLAGS:
    sea-orm-cli migrate {{FLAGS}}

generate: generate-entities
generate-entities:
    sea-orm-cli generate entity -o ./entity/src/entities

build *FLAGS:
    cargo build {{FLAGS}}
