# Sprint Repository - Solana/Anchor Development Journey

## Overview
This repository contains my daily sprint work focusing on Solana blockchain development using the Anchor framework. Each day represents a new learning milestone and practical implementation of Solana smart contracts.

## 🚀 Sprint Progress

### Day 1: Hello Anchor ✅
**Focus**: Structure & Initialization
- Basic Anchor program structure
- Program initialization patterns
- Development environment setup

### Day 2: Counter Program & Variants ✅
**Focus**: Accounts, Constraints, PDAs
- **Basic Counter**: Simple increment/decrement functionality
- **Authority-Only Counter**: Access control mechanisms
- **Resettable Counter**: Reset functionality with permissions
- **Max-Limited Counter**: Boundary constraints implementation
- **Time-based Counter**: Temporal logic integration
- **Multi-Counter per User**: User-specific state management

### Day 3: PDA Writer ✅
**Focus**: PDAs, PDA as Data/Signer, Bump Mechanics
- **User Profile Writer**: User data management with PDAs
- **Authority Delegated Writer**: Delegation mechanisms
- **Nested PDA**: Complex PDA hierarchies

### Day 4: SOL Bank (System Program Transfers) ✅
**Focus**: Native SOL Handling & System Program Integration
- **Basic SOL Bank**: Deposit & withdraw SOL to/from vault PDA
- **P2P Transfer With Escrow Logic**: Alice locks SOL, Bob claims it=
- **Fee-Charging Bank**: Withdrawal fees sent to treasury PDA

### Day 5: Token Vault (Mint, ATA, Transfer) 🚧
**Focus**: SPL Token Operations & Associated Token Accounts
- **Basic Token Vault**: Creates mint + vault PDA for token transfers
- **Authority-Gated Vault**: Only owner/authority can mint tokens to users
- **Per-User ATA Initializer**: Automatic ATA creation and token transfers
- **Locked Vault + Scheduled Claim**: Token escrow with time-locked claims
- **Token Faucet With Rate Limits**: Daily token airdrops with claim tracking
- **Treasury Vault With Multi-Recipient Splits**: Percentage-based token distributions

### Day 6: Escrow 🔁
**Focus**: Multi-Party Atomic Swaps & Complex State Management
- **Basic Token-for-Token Swap**: Atomic exchange between two SPL tokens
- **SOL-for-Token Escrow**: Native SOL and SPL token value-based exchange
- **Token-for-Token with Fixed Amounts**: Rigid swap structures with preset amounts
- **Multi-user Swaps**: Open offer system for OTC-style token exchanges
- **Escrow Vault Reuse**: Single vault for multiple swaps with state optimization
- **Escrow With Fee**: Revenue mechanics with 1-2% fees to treasury

### Day 7: Basic NFT Mint 🔁
**Focus**: NFT Creation & Metadata Management
- **Pure NFT Mint**: Basic 1-supply token creation without metadata
- **Basic Metadata NFT**: NFT with name, symbol, URI via Token Metadata program
- **NFT Mint With Authority Check**: Owner-only minting permissions
- **Time-Limited Mint**: Temporal constraints using Clock sysvar
- **Paid NFT Mint**: Fee-based minting (0.01 SOL) with treasury collection
- **One NFT Per User Limit**: User minting restrictions via PDA tracking

## 📁 Repository Structure
```
├── day-01-hello-anchor/
├── day-02-counter-variants/
├── day-03-pda-writer/
├── day-04-sol-bank/
├── day-05-token-vault/
├── day-06-escrow/
├── day-07-nft-mint/
└── README.md
```

## 🎯 Learning Objectives
- Master Anchor framework fundamentals
- Understand Solana account model and PDAs
- Implement various token standards (SPL, NFT)
- Build complex state management systems
- Create secure smart contract patterns
- Develop real-world DeFi primitives

## 🔧 Setup Instructions
1. Install Rust and Solana CLI
2. Install Anchor CLI
3. Clone this repository
4. Navigate to any day's folder
5. Run `anchor build` and `anchor test`

## 📚 Key Concepts Covered
- **Program Derived Addresses (PDAs)**: Off-curve addresses for program-controlled accounts
- **Cross-Program Invocations (CPIs)**: Calling other programs from your program
- **Associated Token Accounts (ATAs)**: Deterministic token account addresses
- **System Program Integration**: Native SOL transfers and account management
- **Token Program Integration**: SPL token operations and metadata
- **Access Control**: Authority patterns and permission systems
- **State Management**: Complex data structures and account relationships
- **Fee Mechanisms**: Revenue generation and treasury management

## 🤝 Contributing
This is a personal learning repository, but suggestions and improvements are welcome through issues and pull requests.

## 📄 License
MIT License - See LICENSE file for details

---

*Sprint Status: 7/7 Days Complete* ✅