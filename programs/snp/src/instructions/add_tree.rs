use crate::state::Config;
use crate::state::CONFIG_SEEDS;
use crate::MplBubblegum;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;
use spl_account_compression::{program::SplAccountCompression, Noop};

const MAX_TREE_DEPTH: u32 = 24;
const MAX_TREE_BUFFER_SIZE: u32 = 64;

pub fn add_tree(ctx: Context<AddTree>) -> Result<()> {
    CreateTreeConfigCpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&&ctx.accounts.authority.to_account_info())
        .tree_creator(&&ctx.accounts.config.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .max_depth(MAX_TREE_DEPTH)
        .max_buffer_size(MAX_TREE_BUFFER_SIZE)
        .invoke_signed(&[&[
            CONFIG_SEEDS.as_bytes(),
            ctx.accounts.tree_config.key().as_ref(),
            &[ctx.bumps.config],
        ]])?;
	
    Ok(())
}

#[derive(Accounts)]
pub struct AddTree<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
		mut,
		seeds=[
			CONFIG_SEEDS.as_bytes(),
			tree_config.key().as_ref(),
		],
        bump
	)]
    pub config: Account<'info, Config>,
    /// CHECK: This account must be all zeros
    #[account(zero, signer)]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    // programs
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
}
