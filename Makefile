SHELL 			:=bash
.SHELLFLAGS 	:=-euo pipefail -c
.ONESHELL: ;
.EXPORT_ALL_VARIABLES: ;
ifndef DEBUG
.SILENT: ;
endif
.DEFAULT_GOAL	:=build

hack/proxy/pkg/envoy_wasm_header_augmenting_filter_bg.wasm: src/lib.rs
	wasm-pack build --out-dir $(@D)

build: hack/proxy/pkg/envoy_wasm_header_augmenting_filter_bg.wasm
.PHONY: build

integration:
	cd hack
	docker-compose up --build --remove-orphans
.PHONY: integration

deploy: build ; kubectl create -k hack
destroy: ; kubectl delete -k hack
