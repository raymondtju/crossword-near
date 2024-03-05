./deploy.sh

near delete crossword.ferro.testnet ferro.testnet
near create-account crossword.ferro.testnet --masterAccount ferro.testnet --initialBalance 3

near deploy crossword.ferro.testnet ./target/wasm32-unknown-unknown/release/crossword_near.wasm

near call crossword.ferro.testnet new "{\"solution\": \"3f1eced51bac0cb5351bb7c40eacb93146935a0490e7e2d24d177789a3a76e31\"}" --accountId ferro.testnet