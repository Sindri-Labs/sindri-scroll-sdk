helm upgrade -i prover-chunk oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=0.0.1 \
--values ./values/prover-chunk-demo.yaml

helm upgrade -i prover-batch oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=0.0.1 \
--values ./values/prover-batch-demo.yaml

helm upgrade -i prover-bundle oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=0.0.1 \
--values ./values/prover-bundle-demo.yaml
