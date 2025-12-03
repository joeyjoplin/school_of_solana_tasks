//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::errors::TwitterError;
use crate::states::*;
use anchor_lang::solana_program::{hash, system_instruction};
use anchor_lang::AccountSerialize;


pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // TODO: Implement add comment functionality
    require!(comment_content.as_bytes().len() <= COMMENT_LENGTH, TwitterError::CommentTooLong);
      
    let author = &ctx.accounts.comment_author;
    let tweet = &ctx.accounts.tweet;
    let comment_ai = &ctx.accounts.comment;
    let system_program = &ctx.accounts.system_program;

    // 2) Compute the hash OUTSIDE the attribute seeds
    let content_hash = hash::hash(comment_content.as_bytes()).to_bytes();

    // PDA expected by program & tests:
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[
            COMMENT_SEED.as_bytes(),
            author.key().as_ref(),
            content_hash.as_ref(),
            tweet.key().as_ref(),
        ],
        ctx.program_id,
    );

    // Ensure the passed account is exactly that PDA
    require_keys_eq!(
        expected_pda,
        comment_ai.key(),
        anchor_lang::error::ErrorCode::ConstraintSeeds
    );

    // 3) Create account (duplicate should fail with "already in use")
    let space: usize = 8 + Comment::INIT_SPACE;
    let lamports = Rent::get()?.minimum_balance(space);
    let ix = system_instruction::create_account(
        &author.key(),
        &comment_ai.key(),
        lamports,
        space as u64,
        ctx.program_id,
    );

    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            author.to_account_info(),
            comment_ai.to_account_info(),
            system_program.to_account_info(),
        ],
        &[&[
            COMMENT_SEED.as_bytes(),
            author.key().as_ref(),
            content_hash.as_ref(),
            tweet.key().as_ref(),
            &[bump],
        ]],
    )?;

    // 4) Serialize the full Anchor account (discriminador + campos) from offset 0
    let comment_struct = Comment {
        comment_author: author.key(),
        parent_tweet: tweet.key(),
        content: comment_content,
        bump,
    };

    let mut data = comment_ai.try_borrow_mut_data()?;
    {
        // IMPORTANT: AccountSerialize writes the 8-byte discriminator + fields
        let mut cursor = &mut data.as_mut();
        comment_struct.try_serialize(&mut cursor)?;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub comment_author: Signer<'info>,
    pub tweet: Account<'info, Tweet>,
    
    /// CHECK: This account is manually verified in the handler.
    /// The PDA is derived using the seeds:
    /// [COMMENT_SEED, comment_author, sha256(comment_content), tweet].
    /// If it does not exist, it is created via `system_program::create_account`
    /// using `invoke_signed`, and then we manually write the discriminator
    /// and serialized `Comment` data. The `mut` modifier is required
    /// because we create and modify this account in the instruction.
    #[account(mut)] 
    pub comment: UncheckedAccount<'info>,    
    pub system_program: Program<'info, System>,
}
