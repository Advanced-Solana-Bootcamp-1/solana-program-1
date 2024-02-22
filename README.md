# Solana Advanced Bootcamp Guide

This guide provides step-by-step instructions on how to set up and interact with a Solana program example.

## Prerequisites

Before you begin, ensure you have installed the necessary software and tools required to interact with the Solana blockchain.

### Install Solana CLI

1. Download and install the Solana CLI by following the instructions on the [official Solana documentation](https://docs.solanalabs.com/cli/install).

## Getting Started

### Initialize Solana Test Validator

2. Run a Solana test validator locally by executing the following command in your terminal:

```bash
solana-test-validator
```

### Configure Solana CLI

3. Open another terminal window and fetch the current configuration parameters by running:

```bash
solana config get
```

4. Connect to the local validator by setting the URL to localhost:

```bash
solana config set --url localhost
```

5. To connect to the Devnet, change the URL configuration:

```bash
solana config set --url devnet
```

6. For connecting to the Mainnet Beta, set the URL as follows:

```bash
solana config set --url mainnet-beta
```

### Managing Wallets

7. Create a new global wallet file by running:

```bash
solana-keygen new --outfile ~/<id.json>
```

8. Change the global wallet address by specifying the keypair file path:

```bash
solana config set --keypair ~/<FILE_PATH>
```

### Checking Wallet Information

9. To view the global wallet address, use:

```bash
solana address
```

10. Check the balance of the global wallet by:

```bash
solana balance
```

### Performing Transactions

11. To receive an airdrop of 2 SOL into your wallet, execute:

```bash
solana airdrop 2
```
