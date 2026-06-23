#![warn(unused)]
use reth_ethereum::node::api::{ConsensusEngineHandle, PayloadTypes};
use reth_payload_builder::PayloadBuilderHandle;

// main handle for communication in b/w reth and ream.
pub struct RethReamHandle<T: PayloadTypes> {
    pub consensus_engine_handle: ConsensusEngineHandle<T>,
    pub payload_builder_handle: PayloadBuilderHandle<T>,
}

impl<T: PayloadTypes> RethReamHandle<T> {
    pub fn new(
        consensus_engine_handle: ConsensusEngineHandle<T>,
        payload_builder_handle: PayloadBuilderHandle<T>,
    ) -> Self {
        Self { consensus_engine_handle, payload_builder_handle }
    }
}
