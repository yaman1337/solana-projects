use anchor_lang::prelude::*;

declare_id!("4tKNvriW9Wvpqb7ZnCTVC5M62cxpbTkd6gNVwza2ncdu");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        context: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();

        msg!("Greetings from {}", context.program_id);
        msg!("User {user_public_key}'s favorite number is {number}, favorite color is: {color}",);

        msg!("User's hobbies are: {:?}", hobbies);
        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        // makes the favourites account if it doesn't exist already
        init_if_needed,
        // user is the person who signs the transaction
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}
