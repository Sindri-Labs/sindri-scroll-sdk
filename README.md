# Sindri Provers for Scroll SDK

[![Build](https://img.shields.io/github/actions/workflow/status/Sindri-Labs/sindri-scroll-sdk/ci.yaml)](https://github.com/Sindri-Labs/sindri-scroll-sdk/actions)
[![License](https://img.shields.io/github/license/Sindri-Labs/sindri-scroll-sdk)](https://img.shields.io/github/license/Sindri-Labs/sindri-scroll-sdk?style=for-the-badge)

<img src="./media/sindri-gradient-logo.webp" height="160" align="right"/>

#### [Sindri Sign Up](https://sindri.app/signup) | [Scroll SDK Docs](https://docs.scroll.io/en/sdk/) | [Getting Started](#getting-started) | [Development](#internal-development)

Sindri provides automated ZK proving infrastructure, empowering hundreds of teams—including leading Layer 2s and rollups—to launch in minutes instead of months.
Through the Sindri API, developers can seamlessly integrate verifiable computation, reducing time to market, cutting costs, and scaling faster.
Sindri makes zero-knowledge infrastructure simple and accessible, facilitating automation within the most hardware-intensive layer of the ZK app deployment stack.

This repository hosts a proving application image and Helm chart which will route all zkEVM proof generation to the Sindri platform for a Scroll SDK chain.
The following diagram depicts how the Scroll SDK components work together when you use Sindri as a prover.
```mermaid
---
title: Scroll SDK with Sindri Provers
---
flowchart LR
    subgraph settle[Settlement Layer]
        direction LR
    end
    subgraph sequence[Sequencing Layer]
        direction LR
    end
    subgraph prove[Proving Layer]
        direction LR
        C["**Coordinator**"] -.->|Tasks| P@{ shape: processes, label: "**Provers**\nThese components outsource proof generation to Sindri's GPU workers" }
        P -.->|Proofs| C
    end
    subgraph sindri["Scroll Circuits on Sindri"]
        direction TB
        Chunk["**Chunk**
        Verified execution of contiguous blocks (i.e. a zkEVM proof)"]
        Batch["**Batch**
        Intermediate aggregation of multiple chunks"]
        Bundle["**Bundle**
        Final aggregate proof sent to the settlement layer"]
    end
    settle o--o sequence
    sequence o--o prove
    P --> Chunk
    P --> Batch
    P --> Bundle
```

For more information about the Sindri platform, please check out [sindri.app](https://sindri.app/).
The next section ([Getting Started](#getting-started)) shows how to launch a local Scroll SDK devote with provers running on Sindri.
For production deployments, please consult the official [Scroll SDK documentation](https://docs.scroll.io/en/sdk/guides/production-deployment/).

# Getting Started

While this section primarily reiterates [Scroll's Devnet Guide](https://docs.scroll.io/en/sdk/guides/devnet-deployment/), it includes adjustments specifically for Ubuntu environments.
This guide also makes revisions to launch the coordinator service which acts as a gateway between the sequencing layer and Sindri provers.
Since the coordinator requires at least 20 GB of RAM, we recommend you use a machine with at least 32 GB available.

### Prerequisites

You will need to obtain an API key to use Sindri.
If you have not already created an account, you can do so by visiting the [Sindri sign up page](https://sindri.app/signup).
After logging into the [Sindri front-end](https://sindri.app/login), you can create and manage your API Keys within the [API Keys Settings page](https://sindri.app/z/me/page/settings/api-keys).

You should also install `docker`, `kubectl`, `minikube`, `helm`, `node`, and `scrollsdk` as instructed by the official [Scroll SDK documentation](https://docs.scroll.io/en/sdk/guides/devnet-deployment/#prerequisites).

You should initialize your `minikube` environment with three commands:
```bash
minikube start --cpus=12 --memory=32768
minikube addons enable ingress
minikube addons enable ingress-dns
```
In the first command, you are allowing the minikube container to use up to 32 GB of RAM.
The following two commands enable external traffic to reach the minikube cluster so that you can interact with the chain outside the minikube container.

### Obtaining and Configuring Charts

You first need to clone the Scroll SDK repo and navigate to the `devnet/` directory to access some helper scripts.
```bash
git clone git@github.com:scroll-tech/scroll-sdk.git
cd scroll-sdk/devnet
```

This next command manually pulls and extracts the charts for the latest version of Scroll SDK.
```bash
make bootstrap
```
> ⚠️ **Encountering Permission Issues?**<br>
> If you are given any prompts during the execution of the bootstrap command or if `make install` does not successfully execute, then there is likely a root vs. user mismatch between configuration shell scripts and the downloaded files.
> You can solve this by exiting the bootstrap process with `ctrl+c` and running the `config` command separately with `sudo make config`.


### Installing the Helm Chart

In this step, we will launch the sequencing layer, the coordinator, and various visibility services.
Because provers require manual configuration, we will start these services separately.
Note that the standard devnet settings do not include any proving layer services, so you will need to add the following two lines to the end of the `install` command in `scroll-sdk/devnet/Makefile`.
```makefile
 install:
             ...
             # Add the following two rows
             --set coordinator-api.enabled=true \
             --set coordinator-cron.enabled=true \
             ...
```
After that adjustment, you can start the chain by entering the following command in your terminal:
```bash
make install
```
The initialization of all the services associated with your chain can take multiple minutes.
You can monitor the progress and health of all the services via the command `kubectl get pods`.
Once each row in the output of `kubectl get pods` depicts a status of "Ready", your chain is fully operational.

> 🕒 **Waiting on the coordinator?**<br>
> An essential functionality of the coordinator is the validation of proofs that are returned from a prover.
> For this reason, there are two initial pods (`assets-download` and `parameter-download`) which must pull zkEVM circuit artifact files before the final `coordinator-api` pod starts.
> You can watch the progress of the first pod with `kubectl logs coordinator-api-0 -c assets-download`.
> Similarly, you can watch the progress of the trusted setup download via `kubectl logs coordinator-api-0 -c parameter-download`.

### Accessing Services Locally

Now, you should add mappings for the Scroll SDK services to your local DNS resolver by appending the following lines to the end of your `/etc/hosts` file:
```bash
127.0.0.1 frontends.scrollsdk
127.0.0.1 blockscout.scrollsdk
127.0.0.1 bridge-history-api.scrollsdk
127.0.0.1 grafana.scrollsdk
127.0.0.1 l1-devnet.scrollsdk
127.0.0.1 l1-explorer.scrollsdk
127.0.0.1 l2-rpc.scrollsdk
127.0.0.1 rollup-explorer-backend.scrollsdk
127.0.0.1 coordinator-api.scrollsdk
```

You should be able to access the endpoints via your browser.
[This section](https://docs.scroll.io/en/sdk/guides/devnet-deployment/#web-uis) of the Scroll SDK documentation provides an explanation of all the available dashboards.

### 🚀 Launching Sindri Provers

The simplest way to deploy Sindri provers is by following the [Deploy Sindri Provers](demo/README.md) guide helpfully included in the "demo" directory.
Before proceeding to that step, you may find it useful to generate some initial traffic on your chain so that the provers can be sent tasks.
The following command will automatically produce batches and blocks at regular intervals.
Before entering the command, ensure that you are in the `scroll-sdk/devnet/scroll-sdk` directory.
```bash
scrollsdk helper activity -o -t
```

### Cleanup 

You can stop all services with `make delete` and restart with `make install` (assuming you are still in the `scroll-sdk/devnet` directory).
However, you should run `kubectl delete pvc data-l2-rpc-0` after `make delete` to ensure a fresh start.


# Internal Development

This section is intended for Sindri developers.

### Local Build

To run the prover directly in the terminal, first, create your `config.json` file from a template
```bash
cp example.config.json config.json
```
Now edit the `config.json` file to supply your Sindri API key.

Compile and launch the prover via
```bash
cargo run --release
```


### Docker Build

You can build the docker image via
```bash
docker build -t sindri-prover -f docker/Dockerfile .
```
You can then use the example docker compose configuration to launch the container via the following command.
Make sure you follow the initial part of the previous section to get your own `config.json` file.
```bash
docker compose --profile=prover up -d
```
