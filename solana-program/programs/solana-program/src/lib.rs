use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("FP4Rm7yEnHgWdBKJRGdaraE5gBzYhXHMyWHvA9H3uMqC");

#[program]
pub mod solana_program {
    use anchor_lang::solana_program::system_instruction;

    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, celeb: String, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        campaign.celeb_addr = celeb;
        campaign.amount = amount;
        campaign.status = Status::Pending;
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>, status: Status) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.status = status;
        Ok(())
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = user, space = 64 + 64 + 32)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Status {
    Pending,
    Approved,
    Fulfilled,
    Terminated
}
impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

#[account]
pub struct Campaign {
    pub celeb_addr: String,
    pub amount: u64,
    pub status: Status
}
