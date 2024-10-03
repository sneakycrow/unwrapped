set dotenv-load := true

default:
    @just --list

migrate *FLAGS:
    sea-orm-cli migrate {{FLAGS}}

generate: generate-entities
generate-entities:
    sea-orm-cli generate entity -o ./entity/src/entities

build-cargo *FLAGS:
    cargo build {{FLAGS}}

build-api:
    go build -ldflags "-X main.Version=0.1.0" -o dist/api-server ./api

build: build-api

dev-app:
    @just tauri-app/dev

run-api:
    go run -ldflags "-X main.Version=0.1.0" ./api

run: run-api
