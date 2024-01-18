use anchor_lang::prelude::*;

#[account]
pub struct GlobalInfo {
    pub admin: Pubkey,
    pub caller: Pubkey,
    pub current_reward: u64,
    pub halved_times: u64,
    pub last_halved_block_number: u64,
    pub last_mine_time: i64,
}

impl GlobalInfo {
    pub const LEN: usize = 96;
}

#[account]
pub struct MinedInfo {
    pub winner_address: Pubkey,
    pub block_number: u64,
    pub reward_sent: u64,
    pub time_stamp: i64,
}

impl MinedInfo {
    pub const LEN: usize = 56;
}