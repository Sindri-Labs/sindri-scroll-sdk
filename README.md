# sindri-scroll-sdk


<img src="./media/sindri-gradient-logo.webp" height="160" align="right"/>

#### [Sindri Account Creation](https://sindri.app/signup) | [Scroll SDK Docs](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/) | [Getting Started](#getting-started) | [Development](#development)

> Sindri's platform facilitates deploying any nature of zk-rollup from devnet to mainnet.
> This repository hosts the images and Helm charts designed to perform proof generation for Scroll SDK. 

For more information about the Sindri platform, please check out [sindri.app](https://sindri.app/).
The best way to get started with Scroll SDK is by following along with the [devnet tutorial](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/).
The next section will show you how to configure Scroll SDK with actual provers running on Sindri.

# Getting Started

Follow up here after Sindri hosted helm charts.

# Development

To run the prover directly in the terminal, first, create your `config.json` file from a template
```
cp example.config.json config.json
```
Now edit the config to supply your Sindri API key.

Compile and launch the prover via
```
cargo run --release
```


## Docker Image

You can build the docker image locally via
```
docker build -t sindri-prover -f docker/Dockerfile .
```
You can then use the example docker compose configuration to launch the container via the following command.  Make sure you follow the initial part of the previous section to get your own `config.json` file.
```
docker compose --profile=prover up -d
```
