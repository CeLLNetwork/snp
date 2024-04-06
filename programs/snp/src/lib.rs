use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("2Qt3WdLcWJErJkWaqTCwsqWbaFHgq45nhBNk1QEdEfqj");

#[program]
pub mod snp {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn add_tree(ctx: Context<AddTree>) -> Result<()> {
        instructions::add_tree(ctx)
    }

    pub fn mint_cnft(ctx: Context<MintcNFT>, params: MintcNFTParams) -> Result<()> {
        instructions::mint_cnft(ctx, params)
    }

    pub fn set_and_verify_collection(
        ctx: Context<SetAndVerifyCollection>,
        params: SetAndVerifyCollectionParams,
    ) -> Result<()> {
        instructions::set_and_verify_collection(ctx, params)
    }
}

#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}
