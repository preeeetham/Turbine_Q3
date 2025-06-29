# Solana HTTP API Server

A high-performance HTTP server built with Rust that provides RESTful API endpoints for interacting with the Solana blockchain.

## Features

- 🚀 **Fast & Efficient**: Built with Axum and Tokio for high-performance async operations
- 🔗 **Solana Integration**: Direct interaction with Solana blockchain via RPC
- 🌐 **RESTful API**: Clean, intuitive endpoints for blockchain operations
- 🛡️ **Error Handling**: Comprehensive error handling with detailed responses
- 📊 **CORS Support**: Cross-origin resource sharing enabled
- 🔍 **Logging**: Structured logging with tracing
- 🌍 **ngrok Ready**: Can be easily deployed and shared via ngrok

## API Endpoints

### GET `/`
Returns API information and documentation.

### GET `/health`
Health check endpoint.

### GET `/balance/{address}`
Get SOL balance for a Solana address.

**Example:**
```bash
curl http://localhost:8080/balance/11111111111111111111111111111111
```

### GET `/account/{address}`
Get detailed account information for a Solana address.

**Example:**
```bash
curl http://localhost:8080/account/11111111111111111111111111111111
```

### POST `/transfer`
Transfer SOL between addresses.

**Request Body:**
```json
{
  "from": "source_address",
  "to": "destination_address",
  "amount": 0.1,
  "private_key": "base58_encoded_private_key"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "your_source_address",
    "to": "destination_address",
    "amount": 0.1,
    "private_key": "your_base58_private_key"
  }'
```

### GET `/transaction/{signature}`
Get transaction details by signature.

**Example:**
```bash
curl http://localhost:8080/transaction/your_transaction_signature
```

## Installation & Setup

### Prerequisites
- Rust (latest stable version)
- Cargo

### Installation

1. **Clone and navigate to the project:**
   ```bash
   cd http_rust
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Set environment variables (optional):**
   ```bash
   # Create .env file
   echo "SOLANA_RPC_URL=https://api.devnet.solana.com" > .env
   echo "PORT=8080" >> .env
   ```

4. **Run the server:**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080` by default.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SOLANA_RPC_URL` | Solana RPC endpoint | `https://api.devnet.solana.com` |
| `PORT` | Server port | `8080` |

## Deployment with ngrok

To share your API server publicly using ngrok:

1. **Install ngrok** (if not already installed)

2. **Start the server:**
   ```bash
   cargo run
   ```

3. **In another terminal, start ngrok:**
   ```bash
   ngrok http 8080
   ```

4. **Share the ngrok URL** with others to access your API

## Project Structure

```
http_rust/
├── src/
│   ├── main.rs          # Main server setup and routing
│   ├── error.rs         # Error handling and custom error types
│   ├── handlers.rs      # API endpoint handlers
│   └── types.rs         # Request/response types and utilities
├── Cargo.toml           # Dependencies and project configuration
└── README.md           # This file
```

## Dependencies

- **axum**: Modern web framework for Rust
- **tokio**: Async runtime
- **tower-http**: HTTP middleware (CORS)
- **solana-client**: Solana RPC client
- **solana-sdk**: Solana SDK for blockchain operations
- **serde**: Serialization/deserialization
- **tracing**: Structured logging
- **thiserror**: Error handling
- **bs58**: Base58 encoding for Solana keys

## Security Considerations

⚠️ **Important**: This server handles private keys in API requests. In production:

1. Use HTTPS only
2. Implement proper authentication
3. Consider using secure key management
4. Add rate limiting
5. Validate all inputs thoroughly
6. Use environment variables for sensitive configuration

## Development

### Running in Development
```bash
cargo run
```

### Running Tests
```bash
cargo test
```

### Building for Production
```bash
cargo build --release
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Support

For questions or issues, please open an issue in the repository. 