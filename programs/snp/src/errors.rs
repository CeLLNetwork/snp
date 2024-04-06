use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("ContractError: Tree no empty leaf")]
    NoEmptyLeaf,
}
