# Deploy Sindri Provers

This demo builds on the minikube deployment of [Scroll SDK](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/).


## Prerequisites

Before you attempt to launch the proving layers make sure that you have already completed the deployment of all other services in [Scroll SDK](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/) using minikube.
Here are two easy heuristics to ensure that all pods are healthy and you are ready to launch the provers:
* run `kubectl get pods` and ensure that `coordinator-api-0` has a status of `Running` 
* run `scrollsdk test ingress` and ensure that all URLs are reachable.

Clone the [sindri-scroll-sdk](https://github.com/Sindri-Labs/sindri-scroll-sdk) repository and navigate to the `demo/` directory.
```bash
git clone sindri-scroll-sdk
cd demo
```


## Install Sindri Provers

1. Create copies of the values files found in `values/`
   - Change the `-demo.yaml` to `-local.yaml`

2. For each of the 3 values files (`prover-batch-local.yaml`, `prover-chunk-local.yaml`, `prover-bundle-local.yaml`) make the following modifications:
   - Modify `"api_key": "<your-api-key>"` by replacing `<your-api-key>` with your Sindri api key.

3. Identify the desired [version](https://github.com/Sindri-Labs/sindri-scroll-sdk/pkgs/container/sindri-scroll-sdk%2Fhelm%2Fscroll-proving-sindri)

4. Run deploy.sh
   ```bash
   SINDRI_VERSION=0.0.2 ./deploy.sh
   ```
