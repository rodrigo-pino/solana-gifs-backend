use anchor_lang::prelude::*;

declare_id!("HjW9C2WEyLUqvifzkRpefBsYNBgSuCHCyepPY5ovpKw5");

#[program]
pub mod solanagifs {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String, gif_user: Pubkey) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: gif_user,
            gif_upvotes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;

        Ok(())
    }

    pub fn upvote_gif(ctx: Context<UpvoteGif>, index:u32) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.gif_list[index as usize].gif_upvotes += 1;
            
        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u32) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount as u64,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_upvotes: u32
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
