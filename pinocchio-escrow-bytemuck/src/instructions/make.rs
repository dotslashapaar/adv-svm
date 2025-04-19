use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, instruction::Signer, program_error::ProgramError, pubkey, seeds, sysvars::{rent::Rent, Sysvar}, ProgramResult};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::Mint;

use crate::state::Escrow;




#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MakeArgs {
    seed: [u8; 8],
    amount: [u8; 8],
    recevie: [u8; 8],
    bump: u8,
}


impl MakeArgs {
    fn seed(&self) -> u64 {
        u64::from_le_bytes(self.seed)
    }

    fn amount(&self) -> u64 {
        u64::from_le_bytes(self.amount)
    }

    fn receive(&self) -> u64 {
        u64::from_le_bytes(self.recevie)
    }
}

impl TryFrom<&[u8]> for MakeArgs {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            bytemuck::try_from_bytes::<Self>(value)
                .map(|reference| *reference)
                .map_err(|_| ProgramError::InvalidInstructionData)
        
    }
}

pub trait MakeContext<'a> {
    fn make(&self, args: &MakeArgs) -> ProgramResult;
}

impl <'a> MakeContext <'a> for &[AccountInfo] {
    fn make(&self, args: &MakeArgs) -> ProgramResult {
        // all the required accounts for the this instruction
        let [maker, mint_a, mint_b, maker_ata_a, vault, escrow, _system_program, _token_program] = 
        self 
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // doing some checks for accounts
        assert!(maker.is_signer());

        // creating escrow pda 
        let escrow_seeds_with_bump = &[b"escrow", maker.key().as_ref(), &args.seed, &[args.bump]];
        let escrow_derived = pubkey::create_program_address(escrow_seeds_with_bump, &crate::ID)?;

        // checking both created escrow account and input escrow accounts are same
        assert!(escrow_derived == escrow.key().as_ref());
        let bump_ref = &[args.bump];

        // creating signer seeds escrow pda 
        let signer_seeds = seeds!(b"escrow", maker.key().as_ref(), &args.seed, bump_ref);
        let signer = Signer::from(&signer_seeds);

        // Creting escrow pda
        CreateAccount{
            from: maker,
            to: escrow,
            space: Escrow::LEN as u64,
            owner: &crate::ID,
            lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        }.invoke_signed(&[signer])?;

        // Adding(setting-up(path)) the data to state (Read-Write)
        let mut escrow_data = *bytemuck::try_from_bytes_mut::<Escrow>(&mut escrow.try_borrow_mut_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

        // Adding(setting-up(adding)) the data to state (Read-Write)
        escrow_data.clone_from(&Escrow { 
            maker: *maker.key(), 
            mint_a: *mint_a.key(), 
            mint_b: *mint_b.key(), 
            amount: args.amount, 
            receive: args.recevie, 
            seed: args.seed, 
            bump: args.bump,
        });

        // sending mint_a token (maker_ata_a --mint_a--> vault)
        pinocchio_token::instructions::TransferChecked{
            from: maker_ata_a,
            to: vault,
            authority: maker,
            amount: args.amount(),
            mint: mint_a,
            decimals: Mint::from_account_info(mint_a)?.decimals()
        }.invoke()?;

        Ok(())
    }
}

