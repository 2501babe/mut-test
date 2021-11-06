use anchor_lang::prelude::*;

declare_id!("5N5m5EgRmatdkiRQpcdSTZg2wiaSHMaa5WHG1Qmo6W4g");

#[program]
pub mod mut_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("mut-test: initialize");

        msg!("boop before: {:?}", ctx.accounts.boop.boop);
        ctx.accounts.boop.boop = 0;
        msg!("boop after: {:?}", ctx.accounts.boop.boop);

        Ok(())
    }

    pub fn touch(ctx: Context<Touch>) -> ProgramResult {
        msg!("mut-test: touch");

        msg!("boop before: {:?}", ctx.accounts.boop.boop);
        ctx.accounts.boop.boop = 1;
        msg!("boop after: {:?}", ctx.accounts.boop.boop);

        Ok(())
    }

    pub fn slap(ctx: Context<Slap>) -> ProgramResult {
        msg!("mut-test: slap");

        msg!("boop before: {:?}", ctx.accounts.boop.boop);
        let ai = ctx.accounts.boop.to_account_info();
        let mut data = ai.try_borrow_mut_data()?;
        data[8] = 2;
        msg!("boop after: {:?}", ctx.accounts.boop.boop);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [&b""[..]], bump, payer = user)]
    pub boop: Account<'info, Boop>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Touch<'info> {
    // #[account(mut)]
    pub boop: Account<'info, Boop>,
}

#[derive(Accounts)]
pub struct Slap<'info> {
    pub boop: Account<'info, Boop>,
}

#[account]
#[derive(Default)]
pub struct Boop {
    pub boop: u8,
}
