# Sindri Provers for Scroll SDK


<img src="./media/sindri-gradient-logo.webp" height="160" align="right"/>

#### [Sindri Sign Up](https://sindri.app/signup) | [Scroll SDK Docs](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/) | [Getting Started](#getting-started) | [Development](#development)

Sindri provides automated ZK proving infrastructure, empowering hundreds of teams— including leading Layer 2s and rollups—to launch in minutes instead of months.
Through our API, developers can seamlessly integrate verifiable computation, reducing time to market, cutting costs, and scaling faster—just like Stripe revolutionized payments.
Sindri makes zero-knowledge infrastructure simple and accessible, delivering deep automation across every layer of the ZK app deployment stack.

With out-of-the-box tooling, proprietary routing, and optimized performance, Sindri offers a developer experience that mirrors Web2 standards.
We handle complex operations like Kubernetes, auto-scaling, reliability, and on-call management—so our customers don’t have to.
Our infrastructure supports virtually every proof system in the market, ensuring scalability, simplicity, and performance for teams building the future of ZK.

For more information about the Sindri platform, please check out [sindri.app](https://sindri.app/).
The best way to get started with Scroll SDK is by following along with the [devnet tutorial](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/guides/devnet-deployment/).
The next section will show you how to configure Scroll SDK with actual provers running on Sindri.

# Getting Started

1. Obtain a Sindri API key.

After logging into the [Sindri front-end](https://sindri.app/login), you can create and manage your API Keys within the [API Keys Settings page](https://sindri.app/z/me/page/settings/api-keys).

2. 

# Development

To run the prover directly in the terminal, first, create your `config.json` file from a template
```bash
cp example.config.json config.json
```
Now edit the config to supply your Sindri API key.

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
