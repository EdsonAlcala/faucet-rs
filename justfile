set dotenv-load

test:
    echo "Running tests"
    cargo test

build:
    echo "Building"
    cargo near build

clippy:
    echo "Running clippy"
    cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::nursery

deploy:
    echo "Deploying with PHRASE: ${PHRASE}"
    cargo near deploy near-faucet-sepolia.testnet without-init-call network-config testnet sign-with-seed-phrase "${PHRASE}" --seed-phrase-hd-path "m/44'/397'/0'" send
