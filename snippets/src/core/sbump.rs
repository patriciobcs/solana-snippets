/// title: Set PDA Bump
/// description: Sets a PDA bump on an Account
/// platform: anchor
/// prefix: sbump
 
/// snippet requires
use solana_program::ProgramError;

fn sbump() {
    /* snippet */
    __account__.bump = *ctx
        .bumps
        .get("__account__")
        .expect(ProgramError::InvalidAccountData.into());
    /* snippet */
    Ok(())
}
