
use ic_web3::types::H256;
use async_trait::async_trait;

use crate::error::OmnicError;

// each chain client should impl this trait
#[async_trait]
pub trait HomeContract {
    async fn dispatch_message(&self, caller: String, dst_chain: u32, msg: Vec<u8>) -> Result<H256, OmnicError>;
    async fn get_latest_root(&self, height: Option<u64>) -> Result<H256, OmnicError>;
    async fn get_block_number(&self) -> Result<u64, OmnicError>;
}