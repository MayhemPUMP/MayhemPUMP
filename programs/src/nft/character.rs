use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    TokenAccount,
    Token,
};

#[program]
pub mod character_nft {
    use super::*;

    pub fn mint_character(
        ctx: Context<MintCharacter>,
        class: String,
        initial_level: u8,
        initial_attributes: Vec<u8>,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.class = class;
        character.level = initial_level;
        character.attributes = initial_attributes;
        character.owner = ctx.accounts.owner.key();
        character.experience = 0;
        Ok(())
    }

    pub fn level_up(
        ctx: Context<LevelUp>,
        attribute_increases: Vec<u8>,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        require!(character.experience >= level_up_cost(character.level), CharacterError::InsufficientExperience);
        
        character.level = character.level.checked_add(1).ok_or(CharacterError::LevelOverflow)?;
        character.experience = character.experience.checked_sub(level_up_cost(character.level - 1))
            .ok_or(CharacterError::ExperienceUnderflow)?;

        for (i, increase) in attribute_increases.iter().enumerate() {
            if i < character.attributes.len() {
                character.attributes[i] = character.attributes[i].checked_add(*increase)
                    .ok_or(CharacterError::AttributeOverflow)?;
            }
        }

        Ok(())
    }

    pub fn add_experience(
        ctx: Context<AddExperience>,
        amount: u64,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.experience = character.experience.checked_add(amount)
            .ok_or(CharacterError::ExperienceOverflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCharacter<'info> {
    #[account(init, payer = owner, space = Character::LEN)]
    pub character: Account<'info, Character>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LevelUp<'info> {
    #[account(mut, has_one = owner)]
    pub character: Account<'info, Character>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddExperience<'info> {
    #[account(mut)]
    pub character: Account<'info, Character>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Character {
    pub class: String,
    pub level: u8,
    pub attributes: Vec<u8>,
    pub owner: Pubkey,
    pub experience: u64,
}

impl Character {
    pub const LEN: usize = 8 + // discriminator
        32 + // class string
        1 + // level
        32 + // attributes vec
        32 + // owner
        8; // experience
}

#[error_code]
pub enum CharacterError {
    #[msg("Insufficient experience for level up")]
    InsufficientExperience,
    #[msg("Level overflow")]
    LevelOverflow,
    #[msg("Experience underflow")]
    ExperienceUnderflow,
    #[msg("Experience overflow")]
    ExperienceOverflow,
    #[msg("Attribute overflow")]
    AttributeOverflow,
}

fn level_up_cost(current_level: u8) -> u64 {
    1000u64.checked_mul(current_level as u64).unwrap_or(u64::MAX)
}