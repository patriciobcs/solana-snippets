//* title: Clone Keypair
//* description: Clone a keypair
//* platform: client
//* category: system
//* requires
use solana_sdk::signature::Keypair;

pub fn clone_keypair(keypair: &Keypair) -> Keypair {
    /*/* content */*/
    let __cloned_keypair__ = Keypair::from_bytes(&keypair.to_bytes()).unwrap();
    /*/* content */*/
    __cloned_keypair__
}