use anchor_lang::prelude::*;

use crate::state::Config;
use crate::state::CONFIG_SEEDS;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.config.set_inner(Config {
        tree_config: ctx.accounts.tree_config.key(),
        verifier: ctx.accounts.verifier.key(),
        max_supply: 15999999, // -> maxDepth: 24,  maxBufferSize: 64
        empty_leaf: 15999999,
        bump: ctx.bumps.config,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space= Config::INIT_SPACE + 8,
        payer = signer,
        seeds=[
			CONFIG_SEEDS.as_bytes(),
			tree_config.key().as_ref(),
		],
        bump
    )]
    pub config: Box<Account<'info, Config>>,

    /// CHECK
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK
    #[account(mut)]
    pub verifier: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
