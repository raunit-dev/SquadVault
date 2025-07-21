# SquadVault

**SquadVault** is a Solana smart contract program (written in Rust using the Anchor framework) for managing a multisig vault. It allows a group of members to collaboratively manage funds, propose transactions, vote, and execute proposals only when a configurable threshold of votes is met.

---

## Features

- **Multisig Vault Creation:** Initialize a vault with multiple members.
- **Configurable Voting Threshold:** Require a minimum number of votes to execute proposals.
- **Proposal System:** Members can propose fund transfers specifying amount and recipient.
- **Secure Voting:** Only registered members can vote; each member can vote only once per proposal.
- **Execution Logic:** Transfers can only be executed after reaching the required threshold.
- **Error Handling:** Custom errors for threshold validation, double voting, insufficient funds, and more.

---

## Smart Contract Architecture

- **Multisig Struct:** Holds members, threshold, number of proposals, and vault info.
- **Proposal Struct:** Contains proposal ID, amount, recipient, votes, execution status, and voters list.

### Instructions (Core Functions)

- `initialize_multisig`: Create a new multisig vault with members and threshold.
- `create_proposal`: Propose a transfer from the vault.
- `vote_on_proposal`: Vote on an active proposal.
- `execute_proposal`: Execute a proposal after voting threshold is met.

---

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/)
- [Anchor](https://book.anchor-lang.com/) CLI installed
- Solana CLI tools
- Node.js (for running tests)

### Setup

1. **Clone the repo:**
   ```sh
   git clone https://github.com/raunit-dev/SquadVault.git
   cd SquadVault
