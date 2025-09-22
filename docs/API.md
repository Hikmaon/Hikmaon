# HybridChain REST API Documentation

## Overview

HybridChain is a comprehensive blockchain platform featuring proof-of-work consensus, smart contracts, fungible tokens, and certificate management. This documentation provides complete API integration guidelines for developers.

**Base URL:** `http://127.0.0.1:3000`  
**Version:** 1.0  
**Protocol:** HTTP/HTTPS  
**Content-Type:** `application/json`

## Quick Start

### Prerequisites

- HybridChain server running on port 3000
- HTTP client (curl, Postman, or any REST client)
- Basic understanding of blockchain concepts

### Getting Started

1. Start the HybridChain server: `cargo run`
2. Verify the blockchain status: `GET /blockchain/stats`
3. Begin issuing transactions and mining blocks

---

## API Endpoints

### 🎓 Certificate Management

#### Issue Certificate

Creates a new digital certificate and adds it to pending transactions.

**Endpoint:** `POST /certificates/issue`

**Request Body:**

```json
{
  "id": "string",
  "issued_to": "string",
  "description": "string"
}
```

**Response:**

```json
{
  "status": "success",
  "message": "Certificate {id} issued to {issued_to} and added to pending transactions"
}
```

**Example:**

```bash
curl -X POST http://127.0.0.1:3000/certificates/issue \
  -H "Content-Type: application/json" \
  -d '{
    "id": "CERT001",
    "issued_to": "Alice",
    "description": "Blockchain Developer Certificate"
  }'
```

#### Verify Certificate

Validates the existence and authenticity of a certificate.

**Endpoint:** `POST /certificates/verify`

**Request Body:**

```json
{
  "id": "string"
}
```

**Response:**

```json
{
  "status": "success|error",
  "message": "Certificate {id} verified"
}
```

**Example:**

```bash
curl -X POST http://127.0.0.1:3000/certificates/verify \
  -H "Content-Type: application/json" \
  -d '{"id": "CERT001"}'
```

---

### 💰 Token Management

#### Transfer Tokens

Transfers tokens between accounts and creates a blockchain transaction.

**Endpoint:** `POST /tokens/transfer`

**Request Body:**

```json
{
  "from": "string",
  "to": "string",
  "amount": number
}
```

**Response:**

```json
{
  "status": "success|error",
  "message": "Transferred {amount} tokens from {from} to {to} and added to blockchain"
}
```

**Example:**

```bash
curl -X POST http://127.0.0.1:3000/tokens/transfer \
  -H "Content-Type: application/json" \
  -d '{
    "from": "admin",
    "to": "alice",
    "amount": 100
  }'
```

#### Get Token Balance

Retrieves the token balance for a specific account.

**Endpoint:** `GET /tokens/balance/{account}`

**Parameters:**

- `account` (path): Account identifier

**Response:**

```json
{
  "account": "string",
  "balance": number
}
```

**Example:**

```bash
curl http://127.0.0.1:3000/tokens/balance/alice
```

---

### 📦 Blockchain Operations

#### Get All Blocks

Retrieves all blocks in the blockchain.

**Endpoint:** `GET /blocks`

**Response:**

```json
[
  "Block { index: 0, timestamp: ..., transactions: [...], ... }",
  "Block { index: 1, timestamp: ..., transactions: [...], ... }"
]
```

**Example:**

```bash
curl http://127.0.0.1:3000/blocks
```

#### Get Block by Index

Retrieves a specific block by its index.

**Endpoint:** `GET /blocks/{index}`

**Parameters:**

- `index` (path): Block index (0-based)

**Response:**

```json
"Block { index: 2, timestamp: 2025-08-03T01:07:55.837727800Z, ... }"
```

**Example:**

```bash
curl http://127.0.0.1:3000/blocks/0
```

#### Get Blockchain Statistics

Provides comprehensive blockchain metrics and health status.

**Endpoint:** `GET /blockchain/stats`

**Response:**

```json
{
  "total_blocks": number,
  "pending_transactions": number,
  "difficulty": number,
  "is_valid": boolean,
  "latest_hash": "string"
}
```

**Example:**

```bash
curl http://127.0.0.1:3000/blockchain/stats
```

---

### ⛏️ Mining Operations

#### Mine Block

Processes all pending transactions into a new block using proof-of-work.

**Endpoint:** `POST /mine`

**Response:**

```json
{
  "status": "success|info",
  "message": "Successfully mined block with {count} transactions",
  "block_index": number,
  "transactions_count": number
}
```

**Example:**

```bash
curl -X POST http://127.0.0.1:3000/mine
```

#### Get Mining Difficulty

Returns the current proof-of-work difficulty level.

**Endpoint:** `GET /mining/difficulty`

**Response:**

```json
{
  "current_difficulty": number
}
```

**Example:**

```bash
curl http://127.0.0.1:3000/mining/difficulty
```

#### Set Mining Difficulty

Updates the mining difficulty for future blocks.

**Endpoint:** `POST /mining/difficulty`

**Request Body:**

```json
{
  "difficulty": number
}
```

**Response:**

```json
{
  "status": "success",
  "message": "Mining difficulty changed from {old} to {new}"
}
```

**Example:**

```bash
curl -X POST http://127.0.0.1:3000/mining/difficulty \
  -H "Content-Type: application/json" \
  -d '{"difficulty": 4}'
```

---

### ✔️ Validation & Security

#### Validate Blockchain

Performs comprehensive blockchain integrity validation.

**Endpoint:** `GET /blockchain/validate`

**Response:**

```json
{
  "is_valid": boolean,
  "message": "string",
  "details": "string"
}
```

**Example:**

```bash
curl http://127.0.0.1:3000/blockchain/validate
```

#### Validate Block

Validates a specific block's integrity and linkage.

**Endpoint:** `GET /blocks/{index}/validate`

**Parameters:**

- `index` (path): Block index to validate

**Response:**

```json
{
  "is_valid": boolean,
  "message": "Block {index} is valid|validation failed",
  "details": "string"
}
```

**Example:**

```bash
curl http://127.0.0.1:3000/blocks/1/validate
```

#### Validate Chain (Legacy)

Legacy endpoint for tutorial compatibility.

**Endpoint:** `GET /validate`

**Response:**

```json
{
  "status": "success|error",
  "message": "Blockchain is valid.|Blockchain is invalid!"
}
```

---

### 📝 Transaction Management

#### Get Pending Transactions

Retrieves all transactions waiting to be mined.

**Endpoint:** `GET /transactions/pending`

**Response:**

```json
[
  "Transaction { id: \"uuid\", from: \"account\", to: \"account\", amount: 100, ... }",
  "Transaction { id: \"uuid\", from: null, to: \"account\", amount: 0, ... }"
]
```

**Example:**

```bash
curl http://127.0.0.1:3000/transactions/pending
```

---

## Integration Examples

### Complete Workflow Example

```bash
# 1. Check initial blockchain status
curl http://127.0.0.1:3000/blockchain/stats

# 2. Issue a certificate
curl -X POST http://127.0.0.1:3000/certificates/issue \
  -H "Content-Type: application/json" \
  -d '{
    "id": "DEV001",
    "issued_to": "developer@company.com",
    "description": "Senior Blockchain Developer Certification"
  }'

# 3. Transfer tokens
curl -X POST http://127.0.0.1:3000/tokens/transfer \
  -H "Content-Type: application/json" \
  -d '{"from": "admin", "to": "developer@company.com", "amount": 500}'

# 4. Check pending transactions
curl http://127.0.0.1:3000/transactions/pending

# 5. Mine the transactions
curl -X POST http://127.0.0.1:3000/mine

# 6. Validate the blockchain
curl http://127.0.0.1:3000/blockchain/validate

# 7. Verify certificate
curl -X POST http://127.0.0.1:3000/certificates/verify \
  -H "Content-Type: application/json" \
  -d '{"id": "DEV001"}'

# 8. Check final balances
curl http://127.0.0.1:3000/tokens/balance/admin
curl http://127.0.0.1:3000/tokens/balance/developer@company.com
```

### JavaScript Integration

```javascript
class HybridChainClient {
  constructor(baseUrl = "http://127.0.0.1:3000") {
    this.baseUrl = baseUrl;
  }

  async issueCertificate(id, issuedTo, description) {
    const response = await fetch(`${this.baseUrl}/certificates/issue`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id, issued_to: issuedTo, description }),
    });
    return response.json();
  }

  async transferTokens(from, to, amount) {
    const response = await fetch(`${this.baseUrl}/tokens/transfer`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ from, to, amount }),
    });
    return response.json();
  }

  async mineBlock() {
    const response = await fetch(`${this.baseUrl}/mine`, { method: "POST" });
    return response.json();
  }

  async getBlockchainStats() {
    const response = await fetch(`${this.baseUrl}/blockchain/stats`);
    return response.json();
  }
}

// Usage
const client = new HybridChainClient();
await client.issueCertificate("CERT001", "Alice", "Developer Certificate");
await client.mineBlock();
```

### Python Integration

```python
import requests
import json

class HybridChainClient:
    def __init__(self, base_url='http://127.0.0.1:3000'):
        self.base_url = base_url
        self.headers = {'Content-Type': 'application/json'}

    def issue_certificate(self, cert_id, issued_to, description):
        payload = {
            'id': cert_id,
            'issued_to': issued_to,
            'description': description
        }
        response = requests.post(
            f'{self.base_url}/certificates/issue',
            headers=self.headers,
            data=json.dumps(payload)
        )
        return response.json()

    def transfer_tokens(self, from_account, to_account, amount):
        payload = {'from': from_account, 'to': to_account, 'amount': amount}
        response = requests.post(
            f'{self.base_url}/tokens/transfer',
            headers=self.headers,
            data=json.dumps(payload)
        )
        return response.json()

    def mine_block(self):
        response = requests.post(f'{self.base_url}/mine')
        return response.json()

    def get_blockchain_stats(self):
        response = requests.get(f'{self.base_url}/blockchain/stats')
        return response.json()

# Usage
client = HybridChainClient()
client.issue_certificate('CERT001', 'Alice', 'Developer Certificate')
client.mine_block()
```

---

## Error Handling

### Common HTTP Status Codes

- `200 OK`: Request successful
- `400 Bad Request`: Invalid request payload
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

### Error Response Format

```json
{
  "status": "error",
  "message": "Descriptive error message"
}
```

### Common Error Scenarios

1. **Insufficient Token Balance**

   ```json
   {
     "status": "error",
     "message": "Failed to transfer tokens from alice to bob"
   }
   ```

2. **Block Not Found**

   ```json
   {
     "is_valid": false,
     "message": "Block not found",
     "details": "Block index 999 does not exist"
   }
   ```

3. **Invalid Certificate**
   ```json
   {
     "status": "error",
     "message": "Failed to verify certificate INVALID001"
   }
   ```

---

## Best Practices

### Transaction Management

- Always check pending transactions before mining
- Validate blockchain integrity after mining operations
- Monitor token balances after transfers

### Mining Operations

- Mine blocks regularly to process pending transactions
- Adjust difficulty based on network requirements
- Validate blocks after mining for integrity

### Certificate Management

- Use unique, descriptive certificate IDs
- Verify certificates immediately after issuance
- Store certificate details securely off-chain if needed

### Security Considerations

- Validate all user inputs before API calls
- Implement rate limiting for production use
- Monitor blockchain validation status regularly
- Use HTTPS in production environments

---

## System Requirements

### Server Requirements

- Rust 1.70+
- Tokio async runtime
- 512MB+ RAM recommended
- 1GB+ disk space for blockchain data

### Client Requirements

- HTTP/1.1 compatible client
- JSON parsing capabilities
- Network connectivity to server

---

## Support & Resources

### Documentation

- [Rust Documentation](https://doc.rust-lang.org/)
- [Axum Framework](https://docs.rs/axum/)
- [Tokio Runtime](https://tokio.rs/)

### Community

- GitHub Issues: Report bugs and feature requests
- API Updates: Check for latest endpoint additions
- Integration Examples: Community-contributed examples

---

**© 2025 HybridChain Platform. Built with Rust and Axum.**
