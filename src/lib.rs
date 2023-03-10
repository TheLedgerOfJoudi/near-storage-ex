use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenOwners {
    owners_to_tokens: UnorderedMap<String, String>,
    tokens_to_owners: UnorderedMap<String, String>,
    into_storage_key_generator: IntoStorageKeyGenerator,
}

impl Default for TokenOwners {
    fn default() -> Self {
        let mut gen: IntoStorageKeyGenerator = IntoStorageKeyGenerator::default();
        Self {
            owners_to_tokens: UnorderedMap::new(gen.get_into_storage_key()),
            tokens_to_owners: UnorderedMap::new(gen.get_into_storage_key()),
            into_storage_key_generator: gen,
        }
    }
}

#[near_bindgen]
impl TokenOwners {
    pub fn set_info(&mut self, token_id: &String, account_id: &String) {
        self.tokens_to_owners.insert(&token_id, &account_id);
        self.owners_to_tokens.insert(&account_id, &token_id);
    }

    pub fn get_owner(&self, token_id: &String) -> String {
        match self.tokens_to_owners.get(&token_id) {
            Some(owner) => owner,
            None => "No owner".to_string(),
        }
    }

    pub fn get_token(&self, owner_id: &String) -> String {
        match self.owners_to_tokens.get(&owner_id) {
            Some(token) => token,
            None => "No token".to_string(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct IntoStorageKeyGenerator {
    pub ascii: u8,
}

impl Default for IntoStorageKeyGenerator {
    fn default() -> Self {
        Self { ascii: 64 }
    }
}

impl IntoStorageKeyGenerator {
    pub fn get_into_storage_key(&mut self) -> Vec<u8> {
        if self.ascii == 90 {
            self.ascii += 6;
        }
        self.ascii += 1;
        return vec![self.ascii];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    #[test]
    fn run() {
        let context: VMContext = VMContextBuilder::new().context;
        testing_env!(context);
        let mut contract: TokenOwners = TokenOwners::default();
        let token_id: String = "your_token".to_string();
        let owner_id: String = "you.testnet".to_string();
        contract.set_info(&token_id, &owner_id);
        let owner_of_token: String = contract.get_owner(&token_id);
        let token_of_owner: String = contract.get_token(&owner_id);
        assert_eq!(owner_of_token, owner_id);
        assert_eq!(token_of_owner, token_id);
    }
}
