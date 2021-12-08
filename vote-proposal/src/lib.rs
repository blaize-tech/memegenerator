use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, near_bindgen, PanicOnDefault, Metadata};
use near_sdk::collections::Vector;
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ProposalVote {
    account: AccountId,
    metadata: FungibleTokenMetadata,
    uri: String,
    vote_count: i32
}

impl Default for ProposalVote {
    fn default () -> ProposalVote {
        ProposalVote{
            account: "".to_string(),
            metadata: FungibleTokenMetadata{
                spec: "".to_string(),
                name: "".to_string(),
                symbol: "".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 0
            },
            uri: "".to_string(),
            vote_count: 0
        }
    }
}

struct Proposal {
    records: Vector<ProposalVote>,
    token: FungibleToken,
    votes_count_finish: i32
}

#[near_bindgen]
impl Proposal {
    pub fn new() -> Self {
        Self {
            records: Vector::new([]),
            token: FungibleToken,
            votes_count_finish: 3
        }
    }

    pub fn add_proposal(&mut self, creator: AccountId, tokenMetadata: FungibleTokenMetadata, uri: String) {
        self.records.push(&ProposalVote { account: creator, metadata: tokenMetadata, uri: uri,  vote_count: 0 });
    }

    pub fn get_proposal(&self, creator: AccountId) -> ProposalVote {
        match self.records.iter().find(|&&x| x.account == creator) {
            Some(proposalVote) => {
                proposalVote
            },
            None => ProposalVote::default()
        }
    }

    pub fn reset(&mut self) {
        self.records.clear();
        env::log("Reset proposal records".as_bytes());
    }

    pub fn vote(&self, voting: AccountId ) -> bool {
        //TODO: get voting token balance
        let balance = self.token.internal_unwrap_balance_of(&voting);
        println!("{:?}", balance);

        //TODO: add vote counter for each proposal
        self.get_proposal(AccountId).vote_count += 1;

        //TODO: burned token
        if(self.get_proposal(AccountId).vote_count == self.votes_count_finish){
            //TODO: send proposal to NFT template
        }
        return true;
    }
}