# Deploy Sindri Provers

This demo builds on the minikube deployment of [Scroll SDK](https://docs.scroll.xyz/en/sdk/guides/devnet-deployment/).


## Prerequisites

Before you attempt to launch the proving layers make sure that you have already completed the deployment of all other services in [Scroll SDK](https://docs.scroll.xyz/en/sdk/guides/devnet-deployment/) using minikube.
Here are two easy heuristics to ensure that all pods are healthy and you are ready to launch the provers:
* run `kubectl get pods` and ensure that `coordinator-api-0` has a status of `Running` 
* run `scrollsdk test ingress` and ensure that all URLs are reachable.

Clone the [sindri-scroll-sdk](https://github.com/Sindri-Labs/sindri-scroll-sdk) repository and navigate to the `demo/` directory.
```bash
git clone sindri-scroll-sdk
cd demo
```


## Install Sindri Provers

Create your own copy of each of the value files found in `values/`.
These local files will contain your private Sindri API key, but also can be used to fine-tune the behavior of the Sindri provers.
```bash
cd values
cp prover-batch-demo.yaml prover-batch-local.yaml
cp prover-bundle-demo.yaml prover-bundle-local.yaml
cp prover-chunk-demo.yaml prover-chunk-local.yaml
cd ..
```

For each of the three values files (`prover-batch-local.yaml`, `prover-chunk-local.yaml`, `prover-bundle-local.yaml`) you should now modify the `"api_key"` field, replacing `<your-api-key>` with your [Sindri api key](https://sindri.app/z/me/page/settings/api-keys).

Identify the desired [version](https://github.com/Sindri-Labs/sindri-scroll-sdk/pkgs/container/sindri-scroll-sdk%2Fhelm%2Fscroll-proving-sindri).
Generally, the latest stable release is safe to use with the latest Scroll SDK coordinator.

Now, you can launch all Scroll zkEVM proving services with one command via 
   ```bash
   SINDRI_VERSION=0.0.2 ./deploy.sh
   ```
Running `kubectl get pods` should reveal three new services, one for each circuit type.
Assuming your chain has proving tasks, you can monitor the progress of proof generation either via the logs of any prover pod or via Sindri's web front-end after logging in:
* https://sindri.app/z/scroll-tech/batch_prover/proofs
* https://sindri.app/z/scroll-tech/bundle_prover/proofs
* https://sindri.app/z/scroll-tech/chunk_prover/proofs

## Cleanup

You can remove the provers by uninstalling the deployments.
This is done with Helm.  

You can list the deployments using `helm list`.  
You can uninstall a deployment using `helm uninstall <deployment-name>`.  

The following will uninstall the deployments created using the `deploy.sh`.  

```bash
helm uninstall prover-chunk
helm uninstall prover-batch
helm uninstall prover-bundle
```
