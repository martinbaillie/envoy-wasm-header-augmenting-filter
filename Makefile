SHELL 			:=bash
.SHELLFLAGS 	:=-euo pipefail -c
.ONESHELL: ;
.EXPORT_ALL_VARIABLES: ;
ifndef DEBUG
.SILENT: ;
endif
.DEFAULT_GOAL	:=build

RELEASE:=target/wasm32-unknown-unknown/release

$(RELEASE)/envoy_wasm_header_augmenting_filter.wasm: src/lib.rs
	cargo build --target=wasm32-unknown-unknown --release

build: $(RELEASE)/envoy_wasm_header_augmenting_filter.wasm
.PHONY: build

integration:
	cd hack
	docker-compose up --build --remove-orphans
.PHONY: integration

deploy: build ; kubectl create -k .
