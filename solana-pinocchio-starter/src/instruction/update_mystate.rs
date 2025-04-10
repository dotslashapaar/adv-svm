use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};

use crate::{
    error::MyProgramError,
    state::{
        MyState,
        utils::{DataLen, load_acc_mut, load_ix_data},
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateMyStateIxData {
    pub data: [u8; 32],
}

impl DataLen for UpdateMyStateIxData {
    const LEN: usize = core::mem::size_of::<UpdateMyStateIxData>(); // 32 bytes for data
}

pub fn process_update_state(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let my_state = load_acc_mut::<MyState>(unsafe { state_acc.borrow_mut_data_unchecked() })?;

    if my_state.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let ix_data = load_ix_data::<UpdateMyStateIxData>(data)?;

    my_state.update(ix_data)?;

    Ok(())
}
