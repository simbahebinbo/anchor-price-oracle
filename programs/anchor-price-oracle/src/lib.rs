use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_price_oracle {
    use super::*;

    pub fn init_btc_price(ctx: Context<InitBTCPrice>) -> Result<()> {
        let btc_price_account = &mut ctx.accounts.btc_price_account;

        btc_price_account.val = SwitchboardDecimal {
            mantissa: 0,
            scale: 1,
        };

        Ok(())
    }
}

const BTC_PRICE_SEED: &[u8] = b"BTCPrice";


#[derive(Accounts)]
pub struct InitBTCPrice<'info> {
    #[account(
        init,
        seeds = [BTC_PRICE_SEED],
        bump,
        payer = payer,
        space = 16 + 4 + 1,
    )]
    pub btc_price_account: Account<'info, BTCPriceAccountData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct BTCPriceAccountData {
    pub val: SwitchboardDecimal,
}

#[account]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: u32,
}
