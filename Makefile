.PHONY: build docker

GO_TAG?=v4.4.87 
GIT_REV?=ef157528 
ZK_VERSION?=4009e55-e5ddf67

build:
	GO_TAG=${GO_TAG} GIT_REV=${GIT_REV} ZK_VERSION=${ZK_VERSION} cargo build --release

docker:
	docker build -t sindri-prover -f docker/Dockerfile .