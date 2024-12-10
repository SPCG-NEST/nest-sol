use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub id: u64,
    
    pub game_start_slot: u64,
    pub epoch_slot_duration: u64,

    pub current_epoch: u64,
    pub current_epoch_start_slot: u64, 
}
