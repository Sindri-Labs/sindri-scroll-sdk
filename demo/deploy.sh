sindri_version="0.0.1"

helm upgrade -i prover-chunk oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=$sindri_version \
--values ./values/prover-chunk-local.yaml

helm upgrade -i prover-batch oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=$sindri_version \
--values ./values/prover-batch-local.yaml

helm upgrade -i prover-bundle oci://ghcr.io/sindri-labs/sindri-scroll-sdk/helm/scroll-proving-sindri -n default \
--version=$sindri_version \
--values ./values/prover-bundle-local.yaml
