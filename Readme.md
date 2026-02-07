# Aletta

> **The Arbitrum Transaction Debugger that understands both EVM and Stylus**

Aletta turns any Arbitrum transaction hash into a human-readable failure report: exact failing call, decoded error, backtrace, source line (if verified), and actionable fix suggestions — for both Solidity and Stylus contracts.

---

## The Problem

Every developer building on Arbitrum wastes hours debugging reverted transactions. Traditional block explorers show you *that* something failed, but not *why*, *where*, or *how to fix it*.

**Aletta turns debugging from hours into minutes.**

---

## What Makes Aletta Different

- **Stylus-Native**: First debugger that truly understands Arbitrum's WASM contracts alongside EVM
- **Instant Root Cause**: One-line explanation of what failed and why
- **Full Backtrace**: Complete call chain showing exactly how execution reached the failure
- **Source Mapping**: Jump to the exact line in verified contracts where it reverted
- **Actionable Fixes**: Not just "what failed" but "here's how to fix it"
- **Shareable Reports**: Copy link to share debug sessions with your team

---

## Features

### 1. Instant Answer Panel
- Transaction status (success/reverted/out-of-gas)
- Root cause in plain English
- Failing contract + function name
- Complete backtrace of the call chain

### 2. Interactive Call Tree
- Nested visualization of all CALL/DELEGATECALL/STATICCALL operations
- Function names with decoded parameters
- Gas usage, value transfers, and revert reasons per call
- Click any node to inspect inputs/outputs + event logs

### 3. Smart Error Decoding
- Standard revert strings
- Custom errors (when ABI available)
- Common patterns detection:
  - ERC-20 allowance/balance issues
  - Slippage failures
  - Deadline expiration
  - Cross-chain message failures

### 4. Verified Source Integration
For verified contracts:
- Exact file + line number where revert occurred
- Code context with syntax highlighting
- Direct links to source on block explorer

### 5. Stylus-First Support
When transaction touches Stylus contracts:
- Stylus function trace visualization
- Ink (gas) usage breakdown
- One-click copy of replay command:
  ```bash
  cargo stylus trace --tx <HASH> --endpoint <RPC> --use-native-tracer
  ```

### 6. State & Value Tracking
- Token transfers decoded from logs
- Balance changes for involved addresses
- Storage slots touched (when available)

### 7. Fix Suggestions Engine
Intelligent recommendations:
- **Out of gas** → suggests gas limit increase + identifies heavy calls
- **Allowance issues** → shows current vs required allowance
- **Slippage** → compares expected vs actual output
- **Deadline expired** → highlights timestamp mismatch
- **Retryable tickets** → explains cross-chain message failures

### 8. Share & Export
- Permanent shareable report links
- JSON export (receipt + trace + decoded summary)
- GitHub issue template generator

---

## Architecture

### Frontend (Next.js + TypeScript)
- Clean, minimal UI inspired by modern dev tools
- Real-time trace visualization
- Responsive design for desktop and mobile

### Backend (Rust + Axum)
- High-performance RPC interaction
- Advanced trace processing:
  - `debug_traceTransaction` with `callTracer` (primary)
  - `arbtrace_transaction` fallback (for historical blocks)
  - Stylus-aware tracing with `stylusTracer`
- ABI decoding and error resolution
- Report generation and storage

### Tracing Strategy
Aletta handles Arbitrum's tracing complexity correctly:
- Blocks **≥22207818**: Uses `debug_traceTransaction`
- Blocks **≤22207816**: Falls back to `arbtrace_transaction`
- Block **22207817**: Untraceable (handled gracefully)
- **Stylus contracts**: Uses `stylusTracer` with `--use-native-tracer` for maximum RPC compatibility

---

## User Flow

1. **Landing Page**: Enter your name → "Run Aletta"
2. **Analyzer**: 
   - Select network (Arbitrum One / Sepolia / Custom RPC)
   - Paste transaction hash
   - Click "Analyze"
3. **Results**:
   - Instant summary card with root cause
   - Interactive call tree
   - Error details and decoded parameters
   - Stylus trace (if applicable)
   - Fix suggestions
   - Share button

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Frontend | Next.js 14, TypeScript, Tailwind CSS |
| Backend | Rust, Axum, Tokio |
| RPC Interaction | ethers-rs, alloy |
| Tracing | Arbitrum debug_* and arbtrace_* methods |
| Source Verification | Sourcify API |
| Deployment | Vercel (frontend), Railway/Fly.io (backend) |

---

##  Why Aletta


### Technical Excellence
- Handles both modern and historical block tracing
- Graceful fallbacks when RPC capabilities vary
- Stylus-first debugging approach
- Source-level precision for verified contracts


### Innovation
- First debugger that truly understands Stylus
- Backtrace + source mapping + actionable fixes in one tool
- "Replay locally" for Stylus contracts

---

## Getting Started

### Prerequisites
- Node.js 18+
- Rust 1.75+
- Arbitrum RPC endpoint with tracing enabled

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/aletta.git
cd aletta

# Install frontend dependencies
cd frontend
npm install

# Build backend
cd ../backend
cargo build --release
```

### Configuration

Create `.env` files:

**Frontend (.env.local)**:
```env
NEXT_PUBLIC_API_URL=http://localhost:3001
```

**Backend (.env)**:
```env
ARBITRUM_RPC_URL=https://arb1.arbitrum.io/rpc
ARBITRUM_SEPOLIA_RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
DATABASE_URL=postgres://localhost/aletta
```

### Running Locally

```bash
# Terminal 1: Backend
cd backend
cargo run --release

# Terminal 2: Frontend
cd frontend
npm run dev
```

Visit `http://localhost:3000`

---

## API Endpoints

### `POST /analyze`
Analyze a transaction and return debug report.

**Request**:
```json
{
  "name": "Timilehin",
  "chain": "arbitrum-one",
  "txHash": "0x..."
}
```

**Response**:
```json
{
  "reportId": "abc123",
  "status": "reverted",
  "rootCause": "Reverted in swapExactTokensForTokens due to INSUFFICIENT_OUTPUT_AMOUNT",
  "failingFrame": {
    "contract": "0x...",
    "function": "swapExactTokensForTokens",
    "line": 142
  },
  "backtrace": [...],
  "callTree": [...],
  "suggestions": [...]
}
```

### `GET /report/:id`
Retrieve saved report for sharing.

---


## 📄 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🙏 Acknowledgments

- Built for the Arbitrum ecosystem
- Special thanks to Arbitrum's documentation on Stylus tracing

---


**Aletta**: Because every failed transaction deserves a clear explanation.