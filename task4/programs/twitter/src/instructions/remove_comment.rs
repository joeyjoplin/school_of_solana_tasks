//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove comment functionality for the Twitter program
/// 
/// Requirements:
/// - Close the comment account and return rent to comment author
/// 
/// NOTE: No implementation logic is needed in the function body - this 
/// functionality is achieved entirely through account constraints!
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash;
use crate::states::*;

pub fn remove_comment(ctx: Context<RemoveCommentContext>) -> Result<()> {
    let author = &ctx.accounts.comment_author;
    let comment_acc = &ctx.accounts.comment;

    let content_hash = hash::hash(comment_acc.content.as_bytes()).to_bytes();
    let (expected, _bump) = Pubkey::find_program_address(
        &[
            COMMENT_SEED.as_bytes(),
            author.key().as_ref(),
            content_hash.as_ref(),
            comment_acc.parent_tweet.as_ref(),
        ],
        ctx.program_id,
    );

    // Ensure the given account is the correct PDA
    require_keys_eq!(expected, comment_acc.key(), anchor_lang::error::ErrorCode::ConstraintSeeds);

    // Anchor will close `comment` to `comment_author` thanks to the account attribute.
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub comment_author: Signer<'info>,
    // We validate seeds/bump in the handler and close to author.
    #[account(mut, close = comment_author)]
    pub comment: Account<'info, Comment>,
}
