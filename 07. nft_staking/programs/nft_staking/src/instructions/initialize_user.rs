use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]

pub struct InitializeUser<'info> {
    // the user who wants to participate in the staking
    #[account(mut)]
    pub user: Signer<'info>,

    // user's staing account PDA to track the staking data
    #[account(
        init,
        payer = user,
        seeds = [b"user", user.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn initialize_user(&mut self, bumps: InitializeUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0, // start with 0 points
            amount_staked: 0, // initially, no nfts are staked
            bump: bumps.user_account, // store PDA bump
        });
        Ok(())
    }
}
