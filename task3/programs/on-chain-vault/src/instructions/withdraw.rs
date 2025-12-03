//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut,seeds = [b"vault",vault.vault_authority.as_ref()], bump, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,       
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // TODO: Implement withdraw functionality
    require!(amount > 0, VaultError::InsufficientBalance);
    let vault = &ctx.accounts.vault;
    require!(!vault.locked, VaultError::VaultLocked);
    
    let vault_ai = ctx.accounts.vault.to_account_info();
    let authority_ai = ctx.accounts.vault_authority.to_account_info();
    
    
    let current = vault_ai.lamports();
    require!(current >= amount, VaultError::InsufficientBalance);
    // Transfer lamports from vault to vault authority   
    
    **vault_ai.try_borrow_mut_lamports()? -= amount;
    **authority_ai.try_borrow_mut_lamports()? += amount;
    
   
    // Emit withdraw event
    emit!(WithdrawEvent {
        vault: ctx.accounts.vault.key(),
        vault_authority: ctx.accounts.vault_authority.key(),
        amount,
    });
    Ok(())
}
