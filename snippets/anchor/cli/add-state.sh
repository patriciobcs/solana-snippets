#* title: Add State
#* description: Add a new state to your Anchor program (define the name in the terminal with `add=<state_name>` before running the snippet)
#* platform: anchor
#* category: CLI

#* content
state="generic_state"
if ! [ -z ${add+x} ]; then state=$add; fi
mkdir -p src/state
touch src/state/mod.rs
touch src/state/$state.rs
echo "\npub mod $state;\npub use $state::*;" >> src/state/mod.rs
#* content