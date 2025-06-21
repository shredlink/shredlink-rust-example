# Shredlink Rust Example

A Rust example application demonstrating how to use the Shredlink to subscribe to Solana transactions and monitor new token mints on PumpFun.


## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. The project automatically imports `shredlink-proto = "0.1.1"` as specified in `Cargo.toml`

## Usage

### Running the Application

```bash
cargo run
```

### Configuration

Before running, you need to:

1. **Get Shredlink URL**: Ask for the Shredlink service URL in the Discord channel: https://discord.gg/sskBrcfX
2. **Update the URL**: Replace the `shredlink_url` in `src/main.rs` with your actual Shredlink service URL:
