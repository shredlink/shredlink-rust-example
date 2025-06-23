# Shredlink Go Example

A Go example application demonstrating how to use the Shredlink protocol to subscribe to Solana transactions and monitor new token mints on PumpFun.

## Features

- Subscribe to Solana transactions via Shredlink protocol
- Monitor new token mints on PumpFun in real-time
- Extract and display mint addresses and transaction hashes
- Concurrent processing with Go routines
- Clean error handling and graceful shutdown
- Context-based cancellation support

## Dependencies

This project requires the following key dependencies:

- **google.golang.org/grpc** - gRPC client/server implementation for Go
- **google.golang.org/protobuf** - Protocol buffer support for Go
- **github.com/mr-tron/base58** - Base58 encoding/decoding for Solana addresses
- **protoc** - Protocol buffer compiler

## Installation

### Prerequisites

1. **Go 1.21+**: Make sure you have Go installed on your system
2. **Protocol Buffers Compiler**: Install `protoc`
   ```bash
   # macOS
   brew install protobuf
   
   # Ubuntu/Debian
   sudo apt install protobuf-compiler
   
   # Or download from: https://github.com/protocolbuffers/protobuf/releases
   ```

### Setup

1. Clone this repository
2. Initialize Go modules and install dependencies:

```bash
go mod tidy
```

3. Generate the Go gRPC code from the proto file:

```bash
chmod +x generate_proto.sh
./generate_proto.sh
```

This will create a `generated/` directory with the necessary Go files (`shredlink.pb.go` and `shredlink_grpc.pb.go`)

## Usage

### Running the Application

```bash
go run main.go
```

### Configuration

Before running, you need to:

1. **Get Shredlink URL**: Ask for the Shredlink service URL in the Discord channel: https://discord.gg/TR2pxMTz
2. **Update the URL**: Replace the `defaultShredlinkURL` in `main.go` with your actual Shredlink service URL:

```go
const defaultShredlinkURL = "your-shredlink-url:port"
```

### What it does

The application:

1. Connects to the Shredlink service via gRPC
2. Sets up graceful shutdown handling (Ctrl+C)
3. Subscribes to transactions involving the PumpFun program
4. Monitors for new token mints in real-time using streaming gRPC
5. Displays mint addresses and transaction hashes with emojis for better readability

### Example Output

```
🚀 Starting Shredlink PumpFun Monitor (Go)
🔗 Connecting to Shredlink service at localhost:50051...
👀 Monitoring PumpFun new token mints...
--------------------------------------------------
🪙 NEW MINT: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgHU8 | TX HASH: 5KJp89623DjczQzh...
🪙 NEW MINT: 9yQNf21E3rRxsQNxYBgfNrqEhKvXFhpHMKGd5qRrLUo2 | TX HASH: 3mHxBv45821ZsjNm...
```

## Code Structure

- `main.go` - Main application entry point with streaming gRPC transaction subscription
- `createPumpFunRequest()` - Configures the subscription filter for PumpFun mints
- `extractMintAddress()` - Extracts mint addresses from transaction data
- `subscribeEntriesExample()` - Alternative example for subscribing to entries
- `runPumpFunMonitor()` - Core monitoring logic with proper error handling

## Error Handling

The application includes comprehensive error handling for:
- gRPC connection and streaming errors
- Context cancellation (Ctrl+C)
- Signal handling for graceful shutdown
- Transaction processing errors

## Notes

- The application uses Go's context package for cancellation and timeout handling
- Concurrent processing with goroutines for signal handling
- The PumpFun program ID is hardcoded as `TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM`
- Make sure to have a stable connection to the Shredlink service
- The application includes an alternative example for subscribing to entries
- Use Ctrl+C to gracefully shutdown the application

## Quick Start

```bash
# Clone the repository and navigate to it
git clone <repository-url>
cd shredlink-go-example

# Install dependencies
go mod tidy

# Generate gRPC code from proto file
chmod +x generate_proto.sh
./generate_proto.sh

# Update the Shredlink URL in main.go, then run
go run main.go
```

## Project Files

- `shredlink.proto` - Protocol buffer definition for Shredlink service
- `generate_proto.sh` - Shell script to generate Go gRPC code
- `main.go` - Main application with PumpFun monitoring
- `go.mod` - Go module definition and dependencies
- `generated/` - Auto-generated gRPC code (created by generate_proto.sh)

## Building

To build a binary:

```bash
go build -o shredlink-monitor main.go
./shredlink-monitor
```
