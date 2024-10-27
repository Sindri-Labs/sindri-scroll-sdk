# Deploy Sindri Provers

This demo builds on the mini-kube deployment of [scroll-sdk](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/).


## Prerequisites

1. Before you begin the process below make sure that you have already completed deployment of [scroll-sdk](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/) using mini-kube.

2. Clone the [sindri-scroll-sdk](https://github.com/Sindri-Labs/sindri-scroll-sdk) repository.


## Install Sindri Provers

1. Create copies of the values files found in `values/`
   - Change the `-demo.yaml` to `-local.yaml`

2. For each of the 3 values files (`prover-batch-local.yaml`, `prover-chunk-local.yaml`, `prover-bundle-local.yaml`) make the following modifications:
   - Modify `"api_key": "<your-api-key>"` by replacing `<your-api-key>` with your Sindri api key.

3. Identify the desired version (![GitHub Release](https://img.shields.io/github/v/release/Sindri-Labs/sindri-scroll-sdk?style=flat))

4. Run deploy.sh
   ```bash
   SINDRI_VERSION=v0.0.2 ./deploy.sh
   ```
