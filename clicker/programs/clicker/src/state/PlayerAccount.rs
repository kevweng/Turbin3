use anchor_lang::prelude::*;

#[account]
pub struct PlayerAccount {
    // Public key of the player
    pub player: Pubkey,
    
    // Core player statistics  
    pub total_cookies: u64,
    pub cookies_per_second: u64,
    pub total_clicks: u64,

    // Upgrades
    pub cursor_upgrades: u8,
    pub grandma_upgrades: u8,
    pub farm_upgrades: u8,
    pub mine_upgrades: u8,
    pub factory_upgrades: u8,

    // Upgrade costs (dynamic)
    pub cursor_upgrade_cost: u64,
    pub grandma_upgrade_cost: u64,
    pub farm_upgrade_cost: u64,
    pub mine_upgrade_cost: u64,
    pub factory_upgrade_cost: u64,

    // Timestamps and tracking
    pub last_click_timestamp: i64,
    pub bump: u8,
}

impl PlayerAccount {
    pub const INIT_SPACE: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}
