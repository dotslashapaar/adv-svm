use instructions::{escrow_instructions::EscrowInstruction, make::MakeContext, refund::RefundContext, take::TakeContext};
use pinocchio::{account_info::AccountInfo, entrypoint, program_entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

mod instructions;
mod state;

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("AKiu5e7ynzifZG6vy3gt1pT1HFP2uXscRXexH7s3m38A");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    let (instruction, data) = instruction_data
    .split_first()
    .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(instruction)? {
        EscrowInstruction::Make => accounts.make(&data.try_into()?),
        EscrowInstruction::Take => accounts.take(),
        EscrowInstruction::Refund => accounts.refund()
    }

}

