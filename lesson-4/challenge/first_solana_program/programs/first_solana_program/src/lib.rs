use anchor_lang::prelude::*;

declare_id!("A1N7F27c9hpCWRV4oGYShxn1UvVq1duyvQQZwFmWWpPH");

#[program]
pub mod first_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Store>, name: String, age: u8) -> Result<()> {
        if name.len() > 100 {
            return err!(Error::NameTooLong);
        }

        let user_data = &mut ctx.accounts.user_data;
        user_data.set_name(name)?;  
        user_data.age = age;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, name: Option<String>, age: Option<u8>) -> Result<()> {
        let user_data = &mut ctx.accounts.user_data;

        if let Some(name) = name {
            user_data.set_name(name)?;            
        }
        if let Some(age) = age {
            user_data.age = age;
        }
        Ok(())
    }
}   

#[error_code]
pub enum Error {
    #[msg("Name is too long")]
    NameTooLong
}

#[account]
#[derive(InitSpace)]
pub struct UserData {
    #[max_len(100)]
    name: String,
    age: u8,
}

impl UserData {
    fn set_name(&mut self, name: String) -> Result<()> {
        if name.len() > 100 {
            return err!(Error::NameTooLong);
        }
        self.name = name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Store<'info> {
    #[account(init , payer = user, space = 8 + UserData::INIT_SPACE)]
    pub user_data: Account<'info, UserData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user_data: Account<'info, UserData>,
}

