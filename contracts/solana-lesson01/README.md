# Running the Solana Program

This guide outlines the steps to build, deploy, and monitor the Solana program. Ensure you have the Solana CLI and Rust's cargo installed on your system before proceeding.

## Building the Program

1. Open a terminal and navigate to the root directory of your Solana project.

2. Build the program using the following cargo command:

```bash
cargo build-bpf
```

This command compiles your Rust program into a BPF (Berkeley Packet Filter) bytecode, which is the executable format Solana supports.

## Deploying the Program

1. Once the build process is complete, deploy the program to the Solana blockchain using the Solana CLI. You'll need to replace `<PATH>` with the actual path to the compiled `.so` file located under `/target/deploy` directory.

```bash
solana program deploy <PATH>
```

This command uploads your compiled program to the blockchain and returns a program ID. Note down this program ID as it will be used to interact with your program.

## Monitoring Program Logs

1. To monitor the logs and see the outputs from your program, use the following command. Replace `<PROGRAM_ID>` with the program ID you obtained during the deployment step.

```bash
solana logs <PROGRAM_ID>
```
