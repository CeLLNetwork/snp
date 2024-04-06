use anchor_lang::prelude::*;

#[derive(InitSpace, Debug, Default)]
#[account]
pub struct Config {
    pub bump: u8,
    pub tree_config: Pubkey,
    pub verifier: Pubkey,
    pub max_supply: u64,
    pub empty_leaf: u64,
}

pub const CONFIG_SEEDS: &'static str = "cNFT-config";
