#!/bin/bash


function clean_up {
    # Perform program exit housekeeping
    kill -15 $RELAY_PID 2>/dev/null
    kill -15 $VALIDATOR_PID 2>/dev/null
    kill -15 $COLLATOR_PID 2>/dev/null
    exit
}

trap clean_up SIGHUP SIGINT SIGTERM


# export RUST_LOG=debug,sync=info,afg=info,libp2p_swarm=info,multistream_select=info,libp2p_core=info,sub-libp2p=info,libp2p_tcp=info,wasm_overrides=info,wasm-heap=info,libp2p_ping=info,state=trace,runtime=debug
export RUST_LOG=debug,sync=info,afg=info,libp2p_swarm=info,multistream_select=info,libp2p_core=info,sub-libp2p=info,libp2p_tcp=info,wasm_overrides=info,wasm-heap=info,libp2p_ping=info,runtime=debug

termite -e "/bin/bash -c \"./target/release/polkadot --alice --validator --tmp --chain ./local.json --port 30333  --ws-port 9944 2>/tmp/relay \"" &
RELAY_PID=$!

sleep 5
ID=$(grep -a "Local node identity" /tmp/relay | cut -d ":" -f 5 | tr -d '[:space:]')
# wait $RELAY_PID

echo -n "Alice node ID: $ID"
termite -e "/bin/bash -c \"./target/release/polkadot --bob --validator --tmp --chain ./local.json --port 30334  --ws-port 9945 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$ID 2>/tmp/validator \"" &
VALIDATOR_PID=$!

echo "generating parachain genesis"
./../mangata-node/target/release/parachain-collator export-genesis-state > para-2000-genesis
echo "generating parachain wasm"
./../mangata-node/target/release/parachain-collator export-genesis-wasm > para-2000-wasm 

./../mangata-node/target/release/parachain-collator --alice --collator --force-authoring --tmp --port 40333 --ws-port 8844 -- --execution wasm --chain ../polkadot/roccoco_testnet_relay_chain_chain_spec_raw.json --port 30343 --ws-port 9977 2>/tmp/parachain &

COLLATOR_PID=$!
wait $COLLATOR_PID
# RUST_LOG=debug,sync=info,afg=info,libp2p_swarm=info,multistream_select=info,libp2p_core=info,sub-libp2p=info,libp2p_tcp=info,wasm_overrides=info,wasm-heap=info,libp2p_ping=info
