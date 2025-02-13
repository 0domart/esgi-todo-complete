#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod todo {
    use super::*;

  pub fn close(_ctx: Context<CloseTodo>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.todo.count = ctx.accounts.todo.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.todo.count = ctx.accounts.todo.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeTodo>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.todo.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeTodo<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Todo::INIT_SPACE,
  payer = payer
  )]
  pub todo: Account<'info, Todo>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseTodo<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub todo: Account<'info, Todo>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub todo: Account<'info, Todo>,
}

#[account]
#[derive(InitSpace)]
pub struct Todo {
  count: u8,
}
