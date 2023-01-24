#* title: Setup Openbook
#* description: Adds the Anchor SPL DEX library to your Anchor program and start the test validator with the Openbook DEX program.
#* platform: anchor
#* category: CLI

#cargo add --git "https://github.com/openbook-dex/program" --rev "1be91f2" --features "no-entrypoint" serum_dex

#* content
cargo add anchor-spl --features dex
solana program dump -u m srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX openbook_dex_v3.so
solana-test-validator --bpf-program srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX openbook_dex_v3.so --reset
#* content