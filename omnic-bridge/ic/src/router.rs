use crate::error::{Error, Result};
use crate::pool::{Error as PoolError, Pool};
use crate::token::{Error as TokenError, Operation};
use ic_cdk::export::candid::{CandidType, Deserialize, Nat};
use std::collections::BTreeMap;

// chain_id -> Router
#[derive(Deserialize, CandidType, Clone, Debug)]
pub type BridgeRouters = BTreeMap<u32, Router>; 

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Router {
    pub src_chain: u32;
    pub bridge_addr: String; // bridge address on src chain
    pub pools: BTreeMap<u32, Pool>; // src_pool_id -> Pool
}

impl Router {
    pub fn new(
        src_chain: u32,
        bridge_addr: String,
    ) -> Self {
        Router {
            src_chain,
            bridge_addr,
            pools: BTreeMap::new(),
        }
    }

    pub fn add_liquidity(&mut self, pool_id: u32, amount_ld: u128) {
        let mut pool = self.pools.get_mut(&pool_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        pool.add_liquidity(amount_ld);
    }

    pub fn remove_liquidity(&mut self, pool_id: u32, amount_ld: u128) {
        let mut pool = self.pools.get_mut(&pool_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        if pool.enough_liquidity(amount_ld) {
            pool.remove_liquidity(amount_ld)
        }
    }

    pub fn enough_liquidity(&self, pool_id: u32, amount_ld: u128) -> bool {
        let pool = self.pools.get(&pool_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        pool.enough_liquidity(amount_ld)
    }

    pub fn amount_ld(&self, pool_id: u32, amount_sd: u128) -> u128 {
        let pool = self.pools.get(&pool_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        pool.amount_ld(amount_sd)
    }

    pub fn amount_sd(&self, pool_id: u32, amount_ld: u128) -> u128 {
        let pool = self.pools.get(&pool_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        pool.amount_sd(amount_ld)
    }
}

impl BridgeRouters {
    fn add_liquidity(
        &mut self,
        src_chain_id: u32,
        src_pool_id: u32,
        _to: String,
        amount_ld: u128,
    ) {
        let mut router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        router.add_liquidity(src_pool_id, amount_ld);
    }

    fn remove_liquidity(
        &mut self,
        src_chain_id: u32,
        src_pool_id: u32,
        _from: String,
        amount_ld: u32,
    ) {
        let mut router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        router.remove_liquidity(src_pool_id, amount_ld);
    }

    fn swap(
        &mut self,
        src_chain_id: u32,
        src_pool_id: u32,
        dst_chain_id: u32,
        dst_pool_id: u32,
        amount_sd: u128,
    ) {
        let mut src_router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        let mut dst_router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        let dst_amount_ld = dst_router.amount_ld(dst_pool_id, amount_sd);
        if dst_router.enough_liquidity(dst_pool_id, dst_amount_ld) {
            let src_amount_ld = src_router.amount_ld(src_pool_id, amount_sd);
            src_router.add_liquidity(src_pool_id, src_amount_ld);
            dst_router.remove_liquidity(dst_pool_id, dst_amount_ld);
        }
    }

    fn check_swap(
        &mut self,
        src_chain_id: u32,
        src_pool_id: u32,
        dst_chain_id: u32,
        dst_pool_id: u32,
        amount_sd: u128,
    ) -> bool {
        let mut src_router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        let mut dst_router = match self.get_mut(&src_chain_id) {
            Some(p) => p,
            None => unreachable!(),
        };
        let dst_amount_ld = dst_router.amount_ld(dst_pool_id, amount_sd);
        dst_router.enough_liquidity(dst_pool_id, dst_amount_ld)
    }
}
