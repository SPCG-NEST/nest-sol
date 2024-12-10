use anchor_lang::prelude::*;
use crate::state::Game;

pub fn create_game(ctx: Context<CreateGame>, game_create_args: CreateGameArgs) -> Result<()> {
    ctx.accounts.game.id = game_create_args.game_id;
    ctx.accounts.game.game_start_slot = game_create_args.game_start_slot;
    ctx.accounts.game.epoch_slot_duration = game_create_args.epoch_slot_duration;
    ctx.accounts.game.current_epoch = 0;
    ctx.accounts.game.current_epoch_start_slot = 0;


    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateGameArgs {
    pub game_id: u64,
    pub game_start_slot: u64,
    pub epoch_slot_duration: u64,
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(init, payer = authority, space = 8 + Game::INIT_SPACE)]
    pub game: Account<'info, Game>,

}