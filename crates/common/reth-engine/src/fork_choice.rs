// These functions are here to generate the ForkChoice St

use alloy_primitives::{Address, B256};
use alloy_rpc_types_engine::{ForkchoiceState, PayloadAttributes};
use sha2::{Digest, Sha256};

// Creating the Fork Choice state
// at genesis all 3 of them would be B256::Zero
// Current head's hash
// latest justified block's hash
// latest finalized block' hash
pub fn create_fork_choice_state(
    head_block_hash: B256,
    safe_block_hash: B256,
    finalized_block_hash: B256,
) -> ForkchoiceState {
    ForkchoiceState {
        head_block_hash,
        safe_block_hash,
        finalized_block_hash,
    }
}

// #[test]
// fn attributes_serde() {
//     let attributes = r#"{"timestamp":"0x1235","prevRandao":"0xf343b00e02dc34ec0124241f74f32191be28fb370bb48060f5fa4df99bda774c","suggestedFeeRecipient":"0x0000000000000000000000000000000000000000","withdrawals":null,"parentBeaconBlockRoot":null}"#;
//     let _attributes: EthPayloadAttributes = serde_json::from_str(attributes).unwrap();
// }
// /Users/sahilgill/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/alloy-rpc-types-engine-2.1.0/src/payload.rs
// #[test]
// fn test_payload_id_basic() {
//     // Create a parent block and payload attributes
//     let parent =
//         B256::from_str("0x3b8fb240d288781d4aac94d3fd16809ee413bc99294a085798a589dae51ddd4a")
//             .unwrap();
//     let attributes = EthPayloadAttributes {
//         timestamp: 0x5,
//         prev_randao: B256::from_str(
//             "0x0000000000000000000000000000000000000000000000000000000000000000",
//         )
//         .unwrap(),
//         suggested_fee_recipient: Address::from_str(
//             "0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b",
//         )
//         .unwrap(),
//         withdrawals: None,
//         parent_beacon_block_root: None,
//         slot_number: None,
//         target_gas_limit: None,
//     };

// withdrawals: Option<Vec<Withdrawal>>,
// This represents Validator withdrawl, I am not adding it here currently for our case, can be set to none Simply
pub fn create_payload_attributes(
    timestamp: u64,
    prev_randao: B256,
    suggested_fee_recipient: Address,
    parent_beacon_block_root: Option<B256>,
    slot_number: Option<u64>,
    target_gas_limit: Option<u64>,
) -> PayloadAttributes {
    PayloadAttributes {
        timestamp,
        prev_randao,
        suggested_fee_recipient,
        withdrawals: None,
        parent_beacon_block_root,
        slot_number,
        target_gas_limit,
    }
}

// prev_randao = sha256(parent_lean_block_root || slot_le64)
// timestamp = genesis_time + slot * seconds_per_slot (4 s/slot on lean devnet)
pub fn create_lean_payload_attributes(
    slot: u64,
    parent_lean_block_root: B256,
    genesis_time: u64,
    seconds_per_slot: u64,
) -> PayloadAttributes {
    let prev_randao = {
        let mut h = Sha256::new();
        h.update(parent_lean_block_root.as_slice());
        h.update(slot.to_le_bytes());
        B256::from_slice(&h.finalize())
    };

    create_payload_attributes(
        genesis_time + slot * seconds_per_slot,
        prev_randao,
        Address::ZERO,
        Some(parent_lean_block_root),
        Some(slot),
        None,
    )
}
