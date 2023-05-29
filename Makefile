OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif


VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"

DOCKER_EXEC = docker run -it --rm --name api_docs_generator \
	-v ${CURRENT_PATH}:/app -w /app \
	--entrypoint "/root/.cargo/bin/cargo" \
	-v rust_cargo_registy:/root/.cargo/registry \
	jkutkut/docker4rust

docker:
	docker run -it -v $(CURRENT_PATH):/app -w /app -v rust_cargo_registy:/root/.cargo/registry jkutkut/docker4rust


build_release:
	${DOCKER_EXEC} build --release
	docker build -t jkutkut/api_docs_generator:$(VERSION) .
	docker tag jkutkut/api_docs_generator:$(VERSION) jkutkut/api_docs_generator:latest

push_release:
	docker push jkutkut/api_docs_generator:$(VERSION)
	docker push jkutkut/api_docs_generator:latest