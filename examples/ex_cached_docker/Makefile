VERSION	= v0.0.1
DATE	= $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
.PHONY: all build 

all: build

build: 
	docker build --platform=linux/amd64 \
		-f ./Dockerfile \
		--build-arg VERSION=$(VERSION) \
		--build-arg DATE=$(DATE) \
		-t ex_rust_cached:v0.0.1 .


