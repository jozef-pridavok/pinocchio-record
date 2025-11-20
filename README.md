# Solana Record Program

A lightweight Solana program built with [Pinocchio](https://github.com/febo/pinocchio) framework for efficient on-chain data recording and verification. Inspired by [SPL-Record](https://github.com/solana-program/record).

## Overview

This program provides a simple yet powerful mechanism to store and verify u64 values on-chain with authority control. It allows reading data from other accounts (e.g., SPL Token accounts) and recording their state for later verification.

## Features

- **Initialize Record Accounts**: Create new record accounts with authority control
- **Write U64 Values**: Read and store u64 value from specified account at specified offset
- **Check Addition**: Read a u64 value from an account at a specified offset and add increment u64 and compare with stored value in record account. Fail if the new value is not greater than or equal to the stored value plus the increment.
- **Authority Management**: Transfer authority to new owners
- **Account Closure**: Close accounts and reclaim rent

## Prerequisites

- Rust 1.75 or later
- Solana CLI tools 1.18 or later
- Anchor CLI (optional, for easier deployment)

## Installation

```bash
# Clone the repository
git clone https://github.com/jozef-pridavok/arbitrage-programs
cd arbitrage-programs

# Build the program
cargo build-bpf
```

## Configuration

**IMPORTANT**: Before deploying, you must update the program ID in `record/src/lib.rs`:

1. Deploy the program to get your program ID:
   ```bash
   solana program deploy target/deploy/record.so
   ```

2. Get your program ID:
   ```bash
   solana address -k target/deploy/record-keypair.json
   ```

3. Update the `ID` constant in `record/src/lib.rs` with your program ID bytes

4. Rebuild and redeploy:
   ```bash
   cargo build-bpf
   solana program deploy target/deploy/record.so
   ```

## Program Structure

```
record/
├── src/
│   ├── lib.rs          # Program entrypoint and ID
│   ├── processor.rs    # Instruction processing logic
│   ├── instruction.rs  # Instruction definitions
│   ├── state.rs        # Account state structures
│   └── error.rs        # Custom error types
└── tests/
    └── functional_test.rs  # Integration tests
```

## Instructions

### 1. Initialize

Creates and initializes a new record account with the specified authority.

**Accounts:**
- `[writable]` Record account to initialize
- `[readonly]` Authority account

### 2. WriteU64

Reads a u64 value from an external account and stores it in the record account.

**Accounts:**
- `[writable]` Record account
- `[signer]` Authority account
- `[readonly]` Source account to read from

**Parameters:**
- `offset: u64` - Byte offset in the source account where the u64 value is located

### 3. CheckAdd

Verifies that the value in a source account has increased by at least the specified amount compared to the recorded value.

**Accounts:**
- `[readonly]` Record account
- `[signer]` Authority account
- `[readonly]` Source account to verify

**Parameters:**
- `offset: u64` - Byte offset in the source account
- `addition: u64` - Minimum required increase

### 4. SetAuthority

Transfers authority of the record account to a new owner.

**Accounts:**
- `[writable]` Record account
- `[signer]` Current authority
- `[readonly]` New authority account

### 5. CloseAccount

Closes the record account and transfers remaining lamports to the destination.

**Accounts:**
- `[writable]` Record account to close
- `[signer]` Authority account
- `[writable]` Destination account for lamports

## Testing

Run the test suite:

```bash
cargo test
```

Run specific tests:

```bash
cargo test initialize_success
cargo test check_add_success
```

## Usage Example

The program is designed to work with SPL Token accounts or any other account containing u64 data. A typical use case:

1. Initialize a record account with your authority
2. Use `WriteU64` to record the current amount from a token account (at offset 64)
3. Later, use `CheckAdd` to verify the token balance has increased by the expected amount

This is useful for:
- Escrow services verifying payment receipt
- Staking programs tracking deposit increases
- Any scenario requiring proof of value increase
- MEV bots to check profitability of transactions

## Account Data Structure

```rust
pub struct RecordData {
    pub version: u8,        // Structure version (currently 1)
    pub authority: Pubkey,  // Account authority (32 bytes)
    // Followed by 8 bytes of writable u64 storage
}
```

Total minimum account size: 33 bytes (metadata) + 8 bytes (data) = 41 bytes

## Errors

- `IncorrectAuthority`: Provided authority does not match the recorded authority
- `Overflow`: Arithmetic operation resulted in overflow

## Security Considerations

- Always verify the authority is a signer before allowing state changes
- The program validates all account ownership and initialization states
- Overflow checks are performed on all arithmetic operations
- Account closure properly transfers all lamports to prevent rent loss

## Built With

- [Pinocchio](https://github.com/febo/pinocchio) - Ultra-efficient Solana program framework
- [Bytemuck](https://github.com/Lokathor/bytemuck) - Zero-cost type casting

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
