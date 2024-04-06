use crate::state::Config;
use crate::state::CONFIG_SEEDS;
use crate::MplBubblegum;
use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::SetAndVerifyCollectionCpiBuilder;
// use mpl_bubblegum::types::MetadataArgs;
use spl_account_compression::{program::SplAccountCompression, Noop};

pub fn set_and_verify_collection(
    ctx: Context<SetAndVerifyCollection>,
    params: SetAndVerifyCollectionParams,
) -> Result<()> {
    SetAndVerifyCollectionCpiBuilder::new(&ctx.accounts.mpl_bubblegum_program)
        .tree_config(&ctx.accounts.tree_config)
        .leaf_owner(&ctx.accounts.user)
        .leaf_delegate(&ctx.accounts.user)
        .merkle_tree(&ctx.accounts.merkle_tree)
        .payer(&ctx.accounts.user)
        .tree_creator_or_delegate(&ctx.accounts.config.to_account_info(), true)
        .collection_authority(&ctx.accounts.collection_authority)
        .collection_authority_record_pda(Some(&ctx.accounts.collection_authority_record_pda))
        .collection_mint(&ctx.accounts.collection_mint)
        .collection_metadata(&ctx.accounts.collection_metadata)
        .collection_edition(&ctx.accounts.collection_edition)
        .bubblegum_signer(&ctx.accounts.bubblegum_signer)
        .log_wrapper(&ctx.accounts.log_wrapper)
        .compression_program(&ctx.accounts.compression_program)
        .token_metadata_program(&ctx.accounts.token_metadata_program)
        .system_program(&ctx.accounts.system_program)
        .root(params.root)
        .data_hash(params.data_hash)
        .creator_hash(params.creator_hash)
        .nonce(params.nonce)
        .index(params.index)
        .collection(ctx.accounts.collection.key())
        .invoke_signed(&[&[
            CONFIG_SEEDS.as_bytes(),
            ctx.accounts.tree_config.key().as_ref(),
            &[ctx.bumps.config],
        ]])?;
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetAndVerifyCollectionParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    // metadata: MetadataArgs,
}

#[derive(Accounts)]
pub struct SetAndVerifyCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[
			CONFIG_SEEDS.as_bytes(),
			tree_config.key().as_ref(),
		],
        bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK: merkle tree is safe
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    // collection ********************************
    /// CHECK: merkle tree is safe
    #[account(mut)]
    pub collection_authority: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_authority_record_pda: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_edition: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub bubblegum_signer: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub token_metadata_program: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}
