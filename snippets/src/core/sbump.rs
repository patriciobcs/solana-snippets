/**
 * title: Set PDA Bump
 * description: Sets a PDA bump on an Account
 * platform: anchor
 */
// snippet-requires-start
use solana_program::ProgramError;
// snippet-requires-end

fn sbump() {
    // snippet-body-start
    __account__.bump = *ctx
        .bumps
        .get("__account__")
        .expect(ProgramError::InvalidAccountData.into());
    // snippet-body-end
}
