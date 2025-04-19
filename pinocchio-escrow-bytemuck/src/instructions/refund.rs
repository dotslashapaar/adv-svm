use pinocchio::{account_info::AccountInfo, instruction::Signer, program_error::ProgramError, pubkey, seeds, ProgramResult};
use pinocchio_token::state::Mint;

use crate::state::Escrow;


pub trait RefundContext<'a> {
    fn refund(&self) -> ProgramResult;
}

impl <'a> RefundContext<'a> for &[AccountInfo] {
    fn refund(&self) -> ProgramResult {
        // all the required accounts for the this instruction
        let [maker, mint_a, maker_ata_a, vault, escrow, _system_program, _token_program] =
        self
        else{
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // fetching the data from state (Read-Only)
        let escrow_data = *bytemuck::try_from_bytes::<Escrow>(&escrow.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

        // doing some checks for accounts
        assert!(maker.is_signer());
        assert!(escrow.is_owned_by(&crate::ID));
        assert!(&escrow_data.mint_a == mint_a.key());

        // fetching escrow pda
        let escrow_seeds = &[b"escrow", maker.key().as_ref(), &escrow_data.seed];
        let (escrow_derived, escrow_bump) = 
            pubkey::try_find_program_address(escrow_seeds, &crate::ID)
            .ok_or(ProgramError::InvalidSeeds)?;

        // checking both fetched escrow account and input escrow accounts are same
        assert!(escrow_derived == escrow.key().as_ref());
        let bump_ref = &[escrow_bump];

        // creating signer seeds escrow pda 
        let signer_seeds = seeds!(b"escrow", maker.key(), &escrow_data.seed, bump_ref);
        let signer = Signer::from(&signer_seeds);
        let signer1 = Signer::from(&signer_seeds);

        // sending mint_a token (vault --mint_a--> maker_ata_a)
        pinocchio_token::instructions::TransferChecked{
            from: vault,
            to: maker_ata_a,
            authority: escrow,
            amount: u64::from_le_bytes(escrow_data.amount),
            mint: mint_a,
            decimals: Mint::from_account_info(mint_a)?.decimals(),
        }
        .invoke_signed(&[signer])?;

        // account getting closed
        // Giving rent exempt SOL back to maker 
        pinocchio_token::instructions::CloseAccount{
            account: vault,
            destination: maker,
            authority: escrow
        }
        .invoke_signed(&[signer1])?;
    
        // closing escrow pda and giving rent exempt SOL back to maker
        *maker.try_borrow_mut_lamports()? = maker.lamports().checked_add(escrow.lamports())
                                            .ok_or(ProgramError::ArithmeticOverflow)?;

        *escrow.try_borrow_mut_lamports()? = 0;


        Ok(())
    }
}

