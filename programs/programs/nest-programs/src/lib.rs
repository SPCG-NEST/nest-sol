#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("Efd5xU5BLkrWgjdS1k7MDxtZUZkgwnVeHFnD5VGVcGTx");


mod instructions;
mod state;
mod constant;

use crate::instructions::*;

#[program]
pub mod nest_programs {
    use super::*;

    pub fn create_game_ix(ctx: Context<CreateGame>, game_create_args: CreateGameArgs) -> Result<()> {
        create_game(ctx, game_create_args)
    }   
}