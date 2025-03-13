use crate::staking::StakingConfig;
use crate::system::SystemConfig;
use std::collections::HashMap;

pub trait GovernanceConfig: StakingConfig + SystemConfig {}

pub struct Proposal<T: GovernanceConfig> {
    description: String,
    yes_votes: u32,
    no_votes: u32,
    status: ProposalStatus,
    creator: T::AccountId,
}

#[derive(Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
}

pub struct GovernancePallet<T: GovernanceConfig> {
    pub proposals: HashMap<u32, Proposal<T>>,
    pub votes: HashMap<(T::AccountId, u32), bool>, // (voter, proposal_id) -> vote_type
    next_proposal_id: u32,
}

impl<T: GovernanceConfig> GovernancePallet<T> {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            next_proposal_id: 0,
        }
    }

    // Create a new proposal
    pub fn create_proposal(
        &mut self,
        creator: T::AccountId,
        description: String,
    ) -> Result<u32, &'static str> {
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = Proposal {
            description,
            yes_votes: 0,
            no_votes: 0,
            status: ProposalStatus::Active,
            creator,
        };

        self.proposals.insert(proposal_id, proposal);

        Ok(proposal_id)
    }

    // Vote on a proposal (true = yes, false = no)
    pub fn vote(
        &mut self,
        voter: T::AccountId,
        proposal_id: u32,
        vote_type: bool,
    ) -> Result<(), &'static str> {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if proposal.status != ProposalStatus::Active {
                return Err("Proposal is not active");
            }

            if self.votes.contains_key(&(voter.clone(), proposal_id)) {
                return Err("Voter has already voted");
            }

            self.votes.insert((voter, proposal_id), vote_type);

            if vote_type {
                proposal.yes_votes += 1;
            } else {
                proposal.no_votes += 1;
            }

            Ok(())
        } else {
            Err("Proposal does not exist")
        }
    }

    // Get proposal details
    pub fn get_proposal(&self, proposal_id: u32) -> Option<&Proposal<T>> {
        self.proposals.get(&proposal_id)
    }

    // Finalize a proposal (changes status based on votes)
    pub fn finalize_proposal(&mut self, proposal_id: u32) -> Result<ProposalStatus, &'static str> {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if proposal.status != ProposalStatus::Active {
                return Err("Proposal is not active");
            }

            if proposal.yes_votes > proposal.no_votes {
                proposal.status = ProposalStatus::Approved;
            } else {
                proposal.status = ProposalStatus::Rejected;
            }

            Ok(proposal.status.clone())
        } else {
            Err("Proposal does not exist")
        }
    }

    pub fn get_proposal_details(
        &self,
        proposal_id: u32,
    ) -> Result<(String, T::AccountId), &'static str> {
        if let Some(proposal) = self.proposals.get(&proposal_id) {
            Ok((proposal.description.clone(), proposal.creator.clone()))
        } else {
            Err("Proposal does not exist")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Runtime;

    #[test]
    fn test_governance_should_work() {
        let alice = 1u64;
        let bob = 2u64;
        let charlie = 3u64;

        let mut governance = GovernancePallet::<Runtime>::new();

        // Create a proposal
        let proposal_id = governance
            .create_proposal(alice, "Increase validator rewards".to_string())
            .unwrap();

        let (description, creator) = governance.get_proposal_details(proposal_id).unwrap();
        assert_eq!(description, "Increase validator rewards");
        assert_eq!(creator, alice);

        // Cast votes
        governance.vote(alice, proposal_id, true).unwrap(); // Yes vote
        governance.vote(bob, proposal_id, true).unwrap(); // Yes vote
        governance.vote(charlie, proposal_id, false).unwrap(); // No vote

        // Check proposal status before finalization
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.yes_votes, 2);
        assert_eq!(proposal.no_votes, 1);

        // Finalize proposal
        let status = governance.finalize_proposal(proposal_id).unwrap();
        assert!(matches!(status, ProposalStatus::Approved));

        // Check proposal is now approved
        let finalized_proposal = governance.get_proposal(proposal_id).unwrap();
        assert!(matches!(
            finalized_proposal.status,
            ProposalStatus::Approved
        ));
    }
}
