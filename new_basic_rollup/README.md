
# Solana SVM Rollup

A Rust implementation of Solana Virtual Machine (SVM) rollups for off-chain transaction processing and compute unit optimization.

## Overview

The Solana SVM Rollup project provides a framework for processing Solana transactions off-chain using the Solana Virtual Machine, enabling developers to build rollup solutions that maintain Solana's execution environment while reducing on-chain costs and increasing throughput.

## Features

- **Off-chain Transaction Processing**: Execute Solana transactions through the SVM without submitting them to the mainnet
- **Compute Unit Estimation**: Accurately estimate compute unit requirements for transactions before submission
- **Compute Unit Optimization**: Automatically optimize transactions by adding appropriate compute budget instructions
- **Account Caching**: Efficiently cache account data to minimize RPC calls
- **SVM Integration**: Leverage Solana's native VM for consistent execution behavior


## Usage

### Basic RPC Client Extension

```rust
use solana_client::rpc_client::RpcClient;
use solana_svm_rollup::RpcClientExt;
use solana_sdk::{
    message::Message, 
    signature::Keypair, 
    signer::Signer, 
    system_instruction,
    transaction::Transaction,
};

fn main() {
    // Create a new RPC client
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    
    // Generate or load a keypair
    let keypair = Keypair::new();
    
    // Create a simple transfer instruction
    let transfer_ix = system_instruction::transfer(
        &keypair.pubkey(), 
        &Pubkey::new_unique(), 
        10000
    );
    
    // Create a message with the instruction
    let msg = Message::new(&[transfer_ix], Some(&keypair.pubkey()));
    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    
    // Create a new transaction
    let mut tx = Transaction::new(&[&keypair], msg, blockhash);
    
    // Optimize compute units for the transaction
    let optimized_cu = rpc_client
        .optimize_compute_units_unsigned_tx(&mut tx, &[&keypair])
        .unwrap();
    
    println!("Optimized compute units: {}", optimized_cu);
    
    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();
    
    println!("Transaction signature: {}", signature);
}
```


## API Documentation

### RpcClientExt

Extension trait for `solana_client::rpc_client::RpcClient` with the following methods:

#### `estimate_compute_units_unsigned_tx`

Estimates compute units needed for an unsigned transaction using the rollup SVM.

```rust
fn estimate_compute_units_unsigned_tx<'a, I: Signers + ?Sized>(
    &self,
    unsigned_transaction: &Transaction,
    signers: &'a I,
) -> Result<Vec<u64>, Box<dyn std::error::Error + 'static>>
```


#### `estimate_compute_units_msg`

Estimates compute units for a message by simulating it on the network.

```rust
fn estimate_compute_units_msg<'a, I: Signers + ?Sized>(
    &self,
    msg: &Message,
    signers: &'a I,
) -> Result<u64, Box<dyn std::error::Error + 'static>>
```


#### `optimize_compute_units_unsigned_tx`

Optimizes a transaction by adding a compute budget instruction with double the estimated units.

```rust
fn optimize_compute_units_unsigned_tx<'a, I: Signers + ?Sized>(
    &self,
    unsigned_transaction: &mut Transaction,
    signers: &'a I,
) -> Result<u32, Box<dyn std::error::Error + 'static>>
```


#### `optimize_compute_units_msg`

Optimizes a message by adding a compute budget instruction with a 150 unit buffer.

```rust
fn optimize_compute_units_msg<'a, I: Signers + ?Sized>(
    &self,
    message: &mut Message,
    signers: &'a I,
) -> Result<u32, Box<dyn std::error::Error + 'static>>
```



## Why SVM Rollups?

SVM rollups offer several advantages for Solana developers:

- Process transactions off-chain while maintaining compatibility with Solana's execution environment
- Reduce transaction costs by batching multiple operations into fewer on-chain transactions
- Increase throughput beyond the limits of the base chain
- Maintain the same programming model and tooling as standard Solana development


