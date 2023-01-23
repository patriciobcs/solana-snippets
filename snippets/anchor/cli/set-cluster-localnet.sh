#* title: Set Localnet Cluster
#* description: Change the cluster of your Anchor program to Localnet
#* platform: anchor
#* category: CLI

#* content
network="devnet"
crate_name=$(basename "$PWD")
anchor build
program_id=$(solana address -k "target/deploy/$crate_name-keypair.json")
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id}'");/g' "programs/$crate_name/src/lib.rs"
sed -i '' -e 's/^'${crate_name}' = ".*"/'${crate_name}' = "'${program_id}'"/g' Anchor.toml
anchor build
sed -i '' -e 's/^cluster = ".*"/cluster = "'${network}'"/g' Anchor.toml
solana config set --url "http://localhost:8899"
#* content