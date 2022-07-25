use anchor_lang::prelude::*;

use mango::{ state::{ MangoAccount }, matching::Side};

declare_id!("9PWdBsc63S4PZm5XdezHxz681uweUf94534iKyrXRuy8");

const SEED_PHRASE: &[u8; 10] = b"risk-check";

#[program]
pub mod solana_perpetual_trading_risk_checker {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        market_index: u8,
        valid_range: u64
    ) -> Result<()> {
        let risk_checker_account = &mut ctx.accounts.risk_checker_account;

        risk_checker_account.authority = ctx.accounts.authority.key();
        risk_checker_account.bump = *ctx.bumps.get("risk_checker_account").unwrap();

        risk_checker_account.market_index = market_index;
        risk_checker_account.valid_range = valid_range;

        Ok(())
    }

    pub fn check_risk(
        ctx: Context<CheckRisk>,
        market_type: MarketType,
        order_direction: OrderSide,
        order_amount: i64,
    ) -> Result<()> {
        let MarketInfo {
            perp_position,
            perp_unconsumed_position,
            perp_long_order_quantity,
            perp_short_order_quantity
        } = get_mango_account_info(
            ctx.accounts.mango_program.key,
            ctx.accounts.mango_group.key,
            &ctx.accounts.mango_account,
            ctx.accounts.risk_checker_account.market_index    
        );


        Ok(())
    }
}

pub fn get_mango_account_info(
    mango_program_id: &Pubkey,
    mango_group_id: &Pubkey,
    mango_account_info: &AccountInfo,
    market_index: u8,
) -> MarketInfo {
    let mango_account = MangoAccount::load_checked(
        mango_account_info,
        &mango_program_id,
        &mango_group_id
    ).unwrap();

    let perp_account = mango_account.perp_accounts[market_index as usize];

    let perp_position = perp_account.base_position;
    let perp_unconsumed_position = perp_account.taker_base;
    let perp_long_order_quantity = perp_account.bids_quantity;
    let perp_short_order_quantity = perp_account.asks_quantity;

    return MarketInfo {
        perp_position,
        perp_unconsumed_position,
        perp_long_order_quantity,
        perp_short_order_quantity
    }
}

#[account]
#[derive(Default)]
pub struct RiskCheckerAccount {
    pub authority: Pubkey, // 32
    pub bump: u8, // 1
    pub market_index: u8, // 32

    pub valid_range: u64, // 8
}

#[derive(Accounts)]
#[instruction(market_index: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_PHRASE, [market_index].as_ref(), authority.key().as_ref()],
        space = 300,
        bump
    )]
    pub risk_checker_account: Account<'info, RiskCheckerAccount>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CheckRisk<'info> {
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
        seeds = [SEED_PHRASE, [risk_checker_account.market_index].as_ref(), authority.key().as_ref()],
        bump = risk_checker_account.bump
    )]
    pub risk_checker_account: Account<'info, RiskCheckerAccount>,

    /// CHECK: real-only
    pub mango_program: AccountInfo<'info>,
    pub mango_account: AccountInfo<'info>,
    pub mango_group: AccountInfo<'info>,

    // perpetual market state
    pub perp_market: AccountInfo<'info>,
    pub perp_market_bids: AccountInfo<'info>,
    pub perp_market_asks: AccountInfo<'info>,

    // spot market state
    pub spot_market: AccountInfo<'info>,
    pub spot_market_bids: AccountInfo<'info>,
    pub spot_market_asks: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(u8)]
pub enum MarketType {
    Perp = 0,
    Spot = 1,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(u8)]
pub enum OrderSide {
    Long = 0,
    Short = 1,
}

pub struct MarketInfo {
    perp_position: i64,
    
    // Position that has not yet been consumed from event queue
    perp_unconsumed_position: i64,
    
    // This should never be negative
    perp_long_order_quantity: i64,
    
    // This should never be negative
    perp_short_order_quantity: i64,
}
