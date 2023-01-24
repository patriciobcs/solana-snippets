state="initialize"

mkdir -p src/state
touch src/state/mod.rs
touch src/state/$state.rs

echo "\npub mod $state;\npub use $state::*;" >> src/state/mod.rs
