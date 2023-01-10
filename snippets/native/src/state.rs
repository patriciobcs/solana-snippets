use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct __Account__ {}

impl Sealed for __Account__ {}

impl IsInitialized for __Account__ {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for __Account__ {
    const LEN: usize = 0;

    fn unpack_from_slice(_src: &[u8]) -> Result<Self, ProgramError> {
        Ok(__Account__ {})
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}
