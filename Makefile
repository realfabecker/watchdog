.PHONY: build-release
build-release:
	$(info building cargo release)
	cargo build --release --locked

.PHONY: build-bundle
build-bundle:
	$(info building docker image)
	DOCKER_BUILDKIT=1 docker build --target bundle -t bundle . --output ./dist