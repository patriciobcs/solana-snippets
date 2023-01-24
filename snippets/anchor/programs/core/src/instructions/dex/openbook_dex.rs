// Based on https://github.dev/clockwork-xyz/examples/blob/31c97d14aae928f44b2f60d04abd5aa9edf4ce26/openbook_crank/programs/openbook_crank/

//* title: Openbook's Struct
//* description: Openbook's structure useful to for CPI
//* platform: anchor
//* category: dex
//* display: vscode

/*/* content */*/
use anchor_lang::prelude::*;

#[derive(Debug, Clone)]
pub struct OpenBookDex;

impl Id for OpenBookDex {
    fn id() -> Pubkey {
        anchor_spl::dex::ID
    }
}
/*/* content */*/