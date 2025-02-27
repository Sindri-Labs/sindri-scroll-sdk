.PHONY: build docker

GO_TAG?=v4.4.89
GIT_REV?=b0bf0a7d 
ZK_VERSION?=4009e55-1c5093c

build:
	GO_TAG=${GO_TAG} GIT_REV=${GIT_REV} ZK_VERSION=${ZK_VERSION} cargo build --release

docker:
	docker build -t sindri-prover -f docker/Dockerfile .