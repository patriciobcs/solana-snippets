use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {}

impl Sealed for Account {}

impl IsInitialized for Account {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for Account {
    const LEN: usize = 42;

    fn unpack_from_slice(_src: &[u8]) -> Result<Self, ProgramError> {
        Ok(Account {})
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}
