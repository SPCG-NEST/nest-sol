use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Nation {
    pub id: u64,
    #[max_len(12)]
    pub name: String,

}
