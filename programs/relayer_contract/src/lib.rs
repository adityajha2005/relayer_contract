use anchor_lang::prelude::*;

declare_id!("nnS8Xe4mgJC1aM46xBoaBcVrQbmHYH859T8EfvGkUy6");

#[program]
pub mod relayer_contract {
    use super::*;

    pub fn relayer_contract(ctx: Context<RelayTx>) -> Result<()> {
        msg!("Relayed transaction received");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RelayTx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


