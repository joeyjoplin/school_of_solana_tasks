//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut,seeds = [b"vault",vault.vault_authority.as_ref()], bump)]
    pub vault: Account<'info, Vault>,    
    pub system_instructions: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // TODO: Implement deposit functionality
    require!(amount > 0, VaultError::InsufficientBalance);
    let user = &ctx.accounts.user;
    let vault = &ctx.accounts.vault;
    // Check if vault is locked
    require!(!vault.locked, VaultError::VaultLocked);
    // Check if user has enough balance
    let user_balance = user.to_account_info().lamports();
    require!(user_balance >= amount, VaultError::InsufficientBalance);
    // Transfer lamports from user to vault
    invoke(
        &transfer(
            &user.key(),
            &vault.key(),
            amount,
        ),
        &[
            user.to_account_info(),
            vault.to_account_info(),            
        ],
    )?;
    // Emit deposit event
    emit!(DepositEvent {
        vault: vault.key(),
        user: user.key(),
        amount,
    });
    Ok(())
}