use constants::{KEY_VERSION, MPC_CONTRACT_ACCOUNT_ID, MPC_PATH, ONE_DAY};
use near_sdk::{env, ext_contract, near, store::LookupMap, AccountId, Gas, Promise};

mod constants;

#[ext_contract(mpc)]
trait MPC {
    fn sign(&self, payload: [u8; 32], path: String, key_version: u32) -> Promise;
}

#[near(contract_state)]
pub struct Contract {
    owner: AccountId,
    recipients: LookupMap<AccountId, u64>, // account and last time they requested tokens
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            recipients: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near]
impl Contract {
    pub fn request_tokens(&mut self, rlp_payload: [u8; 32]) -> Promise {
        let current_time = env::block_timestamp();
        let predecessor = env::predecessor_account_id();

        // check if the predecessor has requested tokens in the last 24 hours
        if let Some(last_request) = self.recipients.get(&predecessor) {
            if current_time < last_request + ONE_DAY {
                panic!("You can only request tokens once every 24 hours");
            }
        }

        // store the current time as the last time the predecessor requested tokens
        self.recipients.insert(predecessor, current_time);

        mpc::ext(MPC_CONTRACT_ACCOUNT_ID.parse().unwrap())
            .with_static_gas(Gas::from_tgas(100))
            .sign(rlp_payload, String::from(MPC_PATH), KEY_VERSION)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_owner() {
        // TODO: implement
    }
}
