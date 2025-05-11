use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    TokenAccount,
    Token,
};

#[program]
pub mod game_match {
    use super::*;

    pub fn create_match(
        ctx: Context<CreateMatch>,
        match_type: MatchType,
        entry_fee: u64,
        max_players: u8,
    ) -> Result<()> {
        let game_match = &mut ctx.accounts.game_match;
        game_match.match_type = match_type;
        game_match.entry_fee = entry_fee;
        game_match.max_players = max_players;
        game_match.players = vec![ctx.accounts.creator.key()];
        game_match.scores = vec![0];
        game_match.status = MatchStatus::Waiting;
        game_match.created_at = Clock::get()?.unix_timestamp;
        game_match.creator = ctx.accounts.creator.key();
        Ok(())
    }

    pub fn join_match(
        ctx: Context<JoinMatch>,
    ) -> Result<()> {
        let game_match = &mut ctx.accounts.game_match;
        require!(game_match.status == MatchStatus::Waiting, MatchError::MatchNotOpen);
        require!(game_match.players.len() < game_match.max_players as usize, MatchError::MatchFull);
        
        game_match.players.push(ctx.accounts.player.key());
        game_match.scores.push(0);

        if game_match.players.len() == game_match.max_players as usize {
            game_match.status = MatchStatus::InProgress;
            game_match.started_at = Some(Clock::get()?.unix_timestamp);
        }

        Ok(())
    }

    pub fn update_score(
        ctx: Context<UpdateScore>,
        score: u32,
    ) -> Result<()> {
        let game_match = &mut ctx.accounts.game_match;
        require!(game_match.status == MatchStatus::InProgress, MatchError::MatchNotInProgress);
        
        if let Some(player_index) = game_match.players.iter().position(|p| p == ctx.accounts.player.key) {
            game_match.scores[player_index] = score;
        } else {
            return Err(MatchError::PlayerNotInMatch.into());
        }

        Ok(())
    }

    pub fn end_match(
        ctx: Context<EndMatch>,
    ) -> Result<()> {
        let game_match = &mut ctx.accounts.game_match;
        require!(game_match.status == MatchStatus::InProgress, MatchError::MatchNotInProgress);
        require!(ctx.accounts.creator.key() == game_match.creator, MatchError::NotMatchCreator);

        game_match.status = MatchStatus::Completed;
        game_match.ended_at = Some(Clock::get()?.unix_timestamp);

        // 在这里可以添加奖励分配逻辑

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMatch<'info> {
    #[account(init, payer = creator, space = GameMatch::LEN)]
    pub game_match: Account<'info, GameMatch>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinMatch<'info> {
    #[account(mut)]
    pub game_match: Account<'info, GameMatch>,
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(mut)]
    pub game_match: Account<'info, GameMatch>,
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct EndMatch<'info> {
    #[account(mut)]
    pub game_match: Account<'info, GameMatch>,
    pub creator: Signer<'info>,
}

#[account]
pub struct GameMatch {
    pub match_type: MatchType,
    pub entry_fee: u64,
    pub max_players: u8,
    pub players: Vec<Pubkey>,
    pub scores: Vec<u32>,
    pub status: MatchStatus,
    pub creator: Pubkey,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MatchType {
    Solo,
    Duo,
    Squad,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MatchStatus {
    Waiting,
    InProgress,
    Completed,
}

impl GameMatch {
    pub const LEN: usize = 8 + // discriminator
        1 + // match_type
        8 + // entry_fee
        1 + // max_players
        32 + // players vec
        32 + // scores vec
        1 + // status
        32 + // creator
        8 + // created_at
        9 + // started_at option
        9; // ended_at option
}

#[error_code]
pub enum MatchError {
    #[msg("Match is not open for joining")]
    MatchNotOpen,
    #[msg("Match is full")]
    MatchFull,
    #[msg("Match is not in progress")]
    MatchNotInProgress,
    #[msg("Player is not in this match")]
    PlayerNotInMatch,
    #[msg("Only match creator can end the match")]
    NotMatchCreator,
}