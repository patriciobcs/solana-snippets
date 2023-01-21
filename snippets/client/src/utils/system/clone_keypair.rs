use solana_sdk::signature::Keypair;

pub fn clone_keypair(keypair: &Keypair) -> Keypair {
	Keypair::from_bytes(&keypair.to_bytes()).unwrap()
}