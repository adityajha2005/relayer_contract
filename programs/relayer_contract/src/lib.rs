use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction; // For system-level instructions like transfer

declare_id!("nnS8Xe4mgJC1aM46xBoaBcVrQbmHYH859T8EfvGkUy6");

#[program]
pub mod relayer_contract {
    use super::*;

    // This is the main function where the relayed transaction will be processed
    pub fn relayer_contract(ctx: Context<RelayTx>, amount: u64, to: Pubkey) -> Result<()> {
        // Create a system transfer instruction
        let ix = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &to,
            amount,
        );

        // Invoke the system instruction
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        msg!("Transaction relayed successfully to {}", to);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RelayTx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The payer will sign the transaction
    pub system_program: Program<'info, System>, // The system program is used to handle system-level transactions (like transfers)
}
