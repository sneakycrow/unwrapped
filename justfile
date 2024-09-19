set dotenv-load := true

default:
    @just --list

migrate *FLAGS:
    sea-orm-cli migrate {{FLAGS}}

generate: generate-entities
generate-entities:
    sea-orm-cli generate entity -o ./entity/src/entities

build *FLAGS:
    cargo build {{FLAGS}}

dev-app:
    @just tauri-app/dev
