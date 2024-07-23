# Homemade Bitcoin CLI

This is a very experimental project, which I will be developing through time, so be aware that anything here should only be used for testing purposes!


### Before you start
> First of all, you will need a bitcoin node which you know the RPC credentials to connect and start using the CLI. If you do not have any node, you can quick start testing by [downloading](https://bitcoin.org/en/download) and [installing](https://bitcoin.org/en/full-node#linux-instructions) the Bitcoin Core software and starting it on Regtest mode (where you won't need to make the initial blockchain download, which, as the date I am typing, can go through several days, as its size comes around to 600GBs).

> For setting your own RPC credentials for the node, you can set the following in /your/path/to/.bitcoin/bitcoin.conf
```
regtest=1
rpcauth=<your-user>:<your-hashed-password>
rpcuser=<your-user>
rpcpassword=<your-plain-pasword>
``` 
> To generate your rpc credentials(rpcauth string), you can use [this tool](https://jlopp.github.io/bitcoin-core-rpc-auth-generator/)

## How to build and use:

1. 
    Make sure you have Cargo installed! If you dont, just use
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
    Or you can check the [Official Rust Documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

2.
    Clone the repository
    ```bash
    git clone git@github.com:gabrielgusn/homemade-bitcoin-cli.git
    # OR https://github.com/gabrielgusn/homemade-bitcoin-cli.git # IF YOU DON'T HAVE YOUR SSH KEYS PROPERLY CONFIGURED

    cd homemade-bitcoin-cli
    ```
3.
    Build the project
    ```bash
    cargo build
    ```
4.
    Set the environment variables with the data related to your node:
    ```bash
    export BTC_RPC_URL=your-node-url \
    export BTC_RPC_USER=your-rpc-user \
    export BTC_RPC_PASS=your-rpc-pass
    ```
5.
    Run the binary (CLI) with the parameter of your choice (currently only available `getblockhash <height>`)
    ```bash
    ./target/debug/homemade-bitcoin-cli getblockhash <height>
    ```
