#![warn(unused)]
use reth_ethereum::{
    node::api::{ConsensusEngineHandle, PayloadTypes},
};
use reth_payload_builder::PayloadBuilderHandle;

// main handle for communication in b/w reth and ream.
pub struct RethReamHandle<T: PayloadTypes> {
    consensus_engine_handle: ConsensusEngineHandle<T>,
    payload_builder_handle: PayloadBuilderHandle<T>,
}

impl<T: PayloadTypes> RethReamHandle<T> {
    fn new(ceh: ConsensusEngineHandle<T>, pbh: PayloadBuilderHandle<T>) -> RethReamHandle<T> {
        RethReamHandle {
            consensus_engine_handle: ceh,
            payload_builder_handle: pbh,
        }
    }
}
