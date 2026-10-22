
use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_program!(registration);

use registration::cpi::{accounts::Initialize, initialize};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: Initialized by the registration program through CPI.
    #[account(
        mut,
        seeds = [b"prereqs", user.key().as_ref()],
        seeds::program = application_program.key(),
        bump
    )]
    pub application_account: UncheckedAccount<'info>,

    pub application_program: Program<'info, registration::program::Q3PreReqsRs>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // Transfer funds from the vault to the user.
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let vault_state_key = self.vault_state.key();
        let bump = [self.vault_state.vault_bump];

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            vault_state_key.as_ref(),
            &bump,
        ]];

        let transfer_ctx = CpiContext::new_with_signer(
            System::id(),
            transfer_accounts,
            signer_seeds,
        );

        transfer(transfer_ctx, amount)?;

        // Register the GitHub username through the registration program.
        let registration_accounts = Initialize {
            user: self.user.to_account_info(),
            account: self.application_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let registration_ctx = CpiContext::new(
            self.application_program.key(),
            registration_accounts,
        );

        initialize(registration_ctx, "Santhoshini2989".to_string())?;

        Ok(())
    }
}
