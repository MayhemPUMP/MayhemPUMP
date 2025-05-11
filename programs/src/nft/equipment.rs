use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    TokenAccount,
    Token,
};

#[program]
pub mod equipment_nft {
    use super::*;

    pub fn mint_equipment(
        ctx: Context<MintEquipment>,
        equipment_type: String,
        rarity: u8,
        stat_bonuses: Vec<i8>,
    ) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        equipment.equipment_type = equipment_type;
        equipment.rarity = rarity;
        equipment.stat_bonuses = stat_bonuses;
        equipment.owner = ctx.accounts.owner.key();
        equipment.equipped_to = None;
        equipment.durability = 100;
        Ok(())
    }

    pub fn equip(
        ctx: Context<EquipItem>,
        character_pubkey: Pubkey,
    ) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        require!(equipment.owner == ctx.accounts.owner.key(), EquipmentError::InvalidOwner);
        require!(equipment.equipped_to.is_none(), EquipmentError::AlreadyEquipped);
        require!(equipment.durability > 0, EquipmentError::BrokenEquipment);

        equipment.equipped_to = Some(character_pubkey);
        Ok(())
    }

    pub fn unequip(
        ctx: Context<UnequipItem>,
    ) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        require!(equipment.owner == ctx.accounts.owner.key(), EquipmentError::InvalidOwner);
        require!(equipment.equipped_to.is_some(), EquipmentError::NotEquipped);

        equipment.equipped_to = None;
        Ok(())
    }

    pub fn repair(
        ctx: Context<RepairItem>,
    ) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        require!(equipment.owner == ctx.accounts.owner.key(), EquipmentError::InvalidOwner);
        require!(equipment.durability < 100, EquipmentError::AlreadyRepaired);

        equipment.durability = 100;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintEquipment<'info> {
    #[account(init, payer = owner, space = Equipment::LEN)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EquipItem<'info> {
    #[account(mut, has_one = owner)]
    pub equipment: Account<'info, Equipment>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnequipItem<'info> {
    #[account(mut, has_one = owner)]
    pub equipment: Account<'info, Equipment>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RepairItem<'info> {
    #[account(mut, has_one = owner)]
    pub equipment: Account<'info, Equipment>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Equipment {
    pub equipment_type: String,
    pub rarity: u8,
    pub stat_bonuses: Vec<i8>,
    pub owner: Pubkey,
    pub equipped_to: Option<Pubkey>,
    pub durability: u8,
}

impl Equipment {
    pub const LEN: usize = 8 + // discriminator
        32 + // equipment_type string
        1 + // rarity
        32 + // stat_bonuses vec
        32 + // owner
        33 + // equipped_to option
        1; // durability
}

#[error_code]
pub enum EquipmentError {
    #[msg("Invalid equipment owner")]
    InvalidOwner,
    #[msg("Equipment is already equipped")]
    AlreadyEquipped,
    #[msg("Equipment is not equipped")]
    NotEquipped,
    #[msg("Equipment is broken")]
    BrokenEquipment,
    #[msg("Equipment is already fully repaired")]
    AlreadyRepaired,
}