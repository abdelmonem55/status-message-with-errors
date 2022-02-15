use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{env, near_bindgen, BorshStorageKey, AccountId};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Records,
    UniqueValues,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    pub records: LookupMap<AccountId, String>,
    pub unique_values: LookupSet<String>,
}

impl Default for StatusMessage {
    fn default() -> self {
        self {
            records: LookupMap::new(StorageKey::Records),
            unique_values: LookupSet::new(StorageKey::UniqueValues),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    /// Returns true if the message is unique
    pub fn set_status(&self, message: String) -> bool {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, message);
        self.unique_values.insert(&message)
    }

    pub fn get_status(&self, account_id: AccountId) -> Option&ltString&gt {
        self.records.get(&account_id);
    }
}