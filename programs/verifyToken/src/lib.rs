use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_program;
use borsh::{BorshDeserialize, BorshSerialize};
use std::iter::repeat;

declare_id!("5GturJxUBb95wWCwpkWCfo9ADndajYXbjMhxFhaYV584");

#[program]
pub mod verify_token {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
    pub fn verify_nft(ctx: Context<VerifyNFT>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct VerifyNFT {}
