use pinocchio::{
    account_info::AccountInfo, get_account_info, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::{error::RecordError, instruction::RecordInstruction, state::RecordData};

fn check_authority(authority_info: &AccountInfo, expected_authority: &Pubkey) -> ProgramResult {
    if expected_authority != authority_info.key() {
        return Err(RecordError::IncorrectAuthority.into());
    }
    if !authority_info.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = RecordInstruction::unpack(input)?;

    match instruction {
        RecordInstruction::Initialize => {
            let data_info = get_account_info!(accounts, 0);
            let authority_info = get_account_info!(accounts, 1);

            let raw_data = &mut data_info.try_borrow_mut_data().unwrap();
            if raw_data.len() < RecordData::WRITABLE_START_INDEX {
                return Err(ProgramError::InvalidAccountData);
            }

            let account_data = bytemuck::try_from_bytes_mut::<RecordData>(
                &mut raw_data[..RecordData::WRITABLE_START_INDEX],
            )
            .map_err(|_| ProgramError::InvalidArgument)?;

            if account_data.is_initialized() {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            account_data.authority = *authority_info.key();
            account_data.version = RecordData::CURRENT_VERSION;

            Ok(())
        }

        RecordInstruction::WriteU64 { offset } => {
            let data_info = get_account_info!(accounts, 0);
            let authority_info = get_account_info!(accounts, 1);
            let read_account_info = get_account_info!(accounts, 2);
            let raw_data = &data_info.try_borrow_data().unwrap();
            if raw_data.len() < RecordData::WRITABLE_START_INDEX {
                return Err(ProgramError::InvalidAccountData);
            }

            let account_data = bytemuck::try_from_bytes::<RecordData>(
                &raw_data[..RecordData::WRITABLE_START_INDEX],
            )
            .map_err(|_| ProgramError::InvalidArgument)?;

            if !account_data.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;

            let raw_data = read_account_info.try_borrow_data().unwrap();
            let data = &raw_data[offset as usize..offset as usize + 8];
            data_info.try_borrow_mut_data().unwrap()
                [RecordData::WRITABLE_START_INDEX..(RecordData::WRITABLE_START_INDEX + 8)]
                .copy_from_slice(data);

            Ok(())
        }

        RecordInstruction::CheckAdd { offset, addition } => {
            let data_info = get_account_info!(accounts, 0);
            let authority_info = get_account_info!(accounts, 1);
            let read_account_info = get_account_info!(accounts, 2);

            let raw_data = &data_info.try_borrow_data().unwrap();
            if raw_data.len() < RecordData::WRITABLE_START_INDEX {
                return Err(ProgramError::InvalidAccountData);
            }
            let account_data = bytemuck::try_from_bytes::<RecordData>(
                &raw_data[..RecordData::WRITABLE_START_INDEX],
            )
            .map_err(|_| ProgramError::InvalidArgument)?;

            if !account_data.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;

            let old_data = &data_info.try_borrow_data()?
                [RecordData::WRITABLE_START_INDEX..(RecordData::WRITABLE_START_INDEX + 8)];
            let old_value = u64::from_le_bytes(old_data.try_into().unwrap());

            let new_data =
                &read_account_info.try_borrow_data()?[(offset as usize)..(offset as usize) + 8];
            let new_value = u64::from_le_bytes(new_data.try_into().unwrap());

            if new_value >= old_value + addition {
                return Ok(());
            }

            Err(ProgramError::UninitializedAccount)
        }

        RecordInstruction::SetAuthority => {
            let data_info = get_account_info!(accounts, 0);
            let authority_info = get_account_info!(accounts, 1);
            let new_authority_info = get_account_info!(accounts, 2);
            let raw_data = &mut data_info.try_borrow_mut_data()?;
            if raw_data.len() < RecordData::WRITABLE_START_INDEX {
                return Err(ProgramError::InvalidAccountData);
            }

            let account_data = bytemuck::try_from_bytes_mut::<RecordData>(
                &mut raw_data[..RecordData::WRITABLE_START_INDEX],
            )
            .map_err(|_| ProgramError::InvalidArgument)?;

            if !account_data.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }

            check_authority(authority_info, &account_data.authority)?;
            account_data.authority = *new_authority_info.key();

            Ok(())
        }
        RecordInstruction::CloseAccount => {
            let data_info = get_account_info!(accounts, 0);
            let authority_info = get_account_info!(accounts, 1);
            let destination_info = get_account_info!(accounts, 2);
            let raw_data = &mut data_info.try_borrow_mut_data()?;
            if raw_data.len() < RecordData::WRITABLE_START_INDEX {
                return Err(ProgramError::InvalidAccountData);
            }

            let account_data = bytemuck::try_from_bytes_mut::<RecordData>(
                &mut raw_data[..RecordData::WRITABLE_START_INDEX],
            )
            .map_err(|_| ProgramError::InvalidArgument)?;

            if !account_data.is_initialized() {
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;

            let destination_starting_lamports = *destination_info.try_borrow_lamports()?;
            let data_lamports = *data_info.try_borrow_lamports()?;
            *destination_info.try_borrow_mut_lamports().unwrap() = destination_starting_lamports
                .checked_add(data_lamports)
                .ok_or(RecordError::Overflow)?;
            *data_info.try_borrow_mut_lamports().unwrap() = 0_u64;

            Ok(())
        }
    }
}
