# Pre-Req Vault — Turbin3 Challenge

A Solana vault program built with Rust and Anchor as part of the Turbin3 prerequisite challenge.

## Challenge Objective

Extend the vault's withdrawal instruction to invoke the Turbin3 registration program through a Cross-Program Invocation (CPI), registering the developer's GitHub username on-chain.

## Features

- Initialize a vault.
- Deposit SOL into the vault.
- Withdraw SOL from the vault.
- Close the vault and withdraw the remaining funds.
- Invoke the Turbin3 registration program through CPI during withdrawal.
- Register the GitHub username `Santhoshini2989`.

## Program IDs

### Vault Program

**Program ID:**

`FT8JUg1yZcFxS5LNRuC1cUw7S426FetrJj4Hy9YFRkDj`

[View on Solana Explorer](https://explorer.solana.com/address/FT8JUg1yZcFxS5LNRuC1cUw7S426FetrJj4Hy9YFRkDj?cluster=devnet)

### Turbin3 Registration Program

**Program ID:**

`TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM`

[View on Solana Explorer](https://explorer.solana.com/address/TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM?cluster=devnet)

## Implementation Details

The main challenge modification integrates the Turbin3 registration program into the vault's withdrawal instruction.

The registration CPI passes the GitHub username `Santhoshini2989` to the registration program.

This demonstrates the use of:

- **Cross-Program Invocation (CPI):** Calling another Solana program from the vault program.
- **Program Derived Addresses (PDAs):** Deriving program-controlled account addresses.
- **Signer seeds:** Authorizing vault operations when required.
- **On-chain registration:** Recording registration data through the external program.

## Tech Stack

- Rust
- Solana
- Anchor Framework
- TypeScript
- Node.js
- pnpm

## Prerequisites

Install the following tools before running the project:

- Rust toolchain
- Solana CLI
- Anchor CLI
- Node.js
- pnpm

## Getting Started

Clone the repository:

```bash
git clone https://github.com/Santhoshini2989/pre-req-vault.git
cd pre-req-vault
```

Install dependencies:

```bash
pnpm install
```

Build the program:

```bash
anchor build
```

## Testing

Run the Anchor test suite:

```bash
anchor test
```

The tests cover vault initialization, deposits, withdrawals, and closing the vault.

**Note:** The registration program allows one registration per wallet. Re-running a registration test for a wallet whose registration PDA already exists may fail because the account has already been initialized.

## Network

The program was developed and tested using Solana Devnet.

- **Vault Program ID:** `FT8JUg1yZcFxS5LNRuC1cUw7S426FetrJj4Hy9YFRkDj`
- **Registration Program ID:** `TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM`

## References

- [Turbin3 Prerequisite Challenge](https://prereq.turbin3.org/#p1-step2)
- [Original Starter Repository](https://github.com/ShrinathNR/pre-req-vault)

## Author

**GitHub:** [Santhoshini2989](https://github.com/Santhoshini2989)

