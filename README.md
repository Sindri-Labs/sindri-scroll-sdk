# sindri-scroll-sdk

## Temporary Usage Instructions

First, create your `config.json` file from a template
```bash
cp example.config.json config.json
```
Now edit the `config.json` file to supply your `SINDRI_API_KEY`.

Compile and launch the prover via
```bash
cargo run --release
```


## Docker Image

You can build the docker image locally via
```bash
docker build -t sindri-prover -f docker/Dockerfile .
```
You can then use the example docker compose configuration to launch the container via the following command.  Make sure you follow the initial part of the previous section to get your own `config.json` file.
```bash
docker compose --profile=prover up -d
```
