1. Configure MiniKube & Install Scroll-SDK ... <see other instructions>

2. Create a copy of `manifests/regcred-demo.yaml` named `manifests/regcred.yaml`
   Edit the `manifests/regcred.yaml` as follows:
   - set the username to your github username
   - set the password to your github password
   - set the auth to the base64 encoded value of `<username>:<password>`
  > note:  This is only needed while the repo is "private"

3. Deploy your regcred manifest
   `kubectl apply -f manifests/regcred.yaml`

4. Create copies of the values files found in `values/`
   - Change the `-demo.yaml` to `-local.yaml`

5. For each of the 3 values files (`prover-batch-local.yaml`, `prover-chunk-local.yaml`, `prover-bundle-local.yaml`) make the following modifications:
   - Modify `"api_key": "<your-api-key>"` by replacing `<your-api-key>` with your Sindri api key.

6. Log into the GitHub container registry
   `helm registry login https://ghcr.io`
   When you are prompted for a username provide your GitHub username.
   When you are prompted for a password provide you Personal Access Token (PAT).
  > note:  This is only needed while the repo is "private"

7. Update deploy.sh with the desired version:
   `sindri_version="0.0.1"`

8. Run deploy.sh
