set dotenv-load := true

default:
    @just --list

alias run := start
alias dev := start
start:
    cargo run -p unwrapped

migrate:
    sea-orm-cli migrate

generate: generate-entities
generate-entities:
    sea-orm-cli generate entity -o ./entity/src/entities