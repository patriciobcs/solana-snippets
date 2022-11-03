/**
 * title: Set PDA Bump
 * description: Sets a PDA bump on an Account
 * platform: anchor
 */
// snippet-suggestion-start
use solana_program::ProgramError;
// snippet-suggestion-end

fn sbump() {
    // snippet-start
    __account__.bump = *ctx
        .bumps
        .get("__account__")
        .expect(ProgramError::InvalidAccountData.into());
    // snippet-end
}
