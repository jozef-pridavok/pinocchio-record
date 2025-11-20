use pinocchio::{
    account_info::AccountInfo, default_allocator, default_panic_handler, program_entrypoint,
    pubkey::Pubkey, ProgramResult,
};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

pub use pinocchio;

// TODO: Replace with your deployed program ID
// To get your program ID after deployment, run: solana address -k target/deploy/record-keypair.json
pub const ID: Pubkey = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)
}

program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();
