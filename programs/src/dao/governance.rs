use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    TokenAccount,
    Token,
};

#[program]
pub mod governance {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        voting_period: i64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.creator = ctx.accounts.creator.key();
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.status = ProposalStatus::Active;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.voting_ends_at = proposal.created_at + voting_period;
        Ok(())
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote: bool,
        voting_power: u64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);
        require!(clock.unix_timestamp < proposal.voting_ends_at, GovernanceError::VotingPeriodEnded);

        if vote {
            proposal.yes_votes = proposal.yes_votes.checked_add(voting_power)
                .ok_or(GovernanceError::VoteOverflow)?;
        } else {
            proposal.no_votes = proposal.no_votes.checked_add(voting_power)
                .ok_or(GovernanceError::VoteOverflow)?;
        }

        Ok(())
    }

    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let clock = Clock::get()?;

        require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);
        require!(clock.unix_timestamp >= proposal.voting_ends_at, GovernanceError::VotingPeriodNotEnded);

        if proposal.yes_votes > proposal.no_votes {
            proposal.status = ProposalStatus::Executed;
            // 在这里添加提案执行逻辑
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        Ok(())
    }

    pub fn cancel_proposal(
        ctx: Context<CancelProposal>,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);
        require!(proposal.creator == ctx.accounts.creator.key(), GovernanceError::NotProposalCreator);

        proposal.status = ProposalStatus::Cancelled;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub creator: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub voting_ends_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Executed,
    Rejected,
    Cancelled,
}

impl Proposal {
    pub const LEN: usize = 8 + // discriminator
        32 + // title string
        256 + // description string
        32 + // creator
        8 + // yes_votes
        8 + // no_votes
        1 + // status
        8 + // created_at
        8; // voting_ends_at
}

#[error_code]
pub enum GovernanceError {
    #[msg("Proposal is not active")]
    ProposalNotActive,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("Vote calculation overflow")]
    VoteOverflow,
    #[msg("Only proposal creator can cancel")]
    NotProposalCreator,
}