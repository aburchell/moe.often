use gloo::storage::{LocalStorage, Storage, errors::StorageError};

use crate::data;
use data::State;

const ONBOARDING_KEY: &str = "OnboardingTokenKey";
const APP_STATE_KEY: &str = "MoeOftenAppState";

fn get_state() -> Result<State, StorageError> {
    LocalStorage::get::<State>(String::from(APP_STATE_KEY))
}

fn set_state(new_state: State) -> Result<State, StorageError> {
    match LocalStorage::set::<State>(String::from(APP_STATE_KEY), new_state.clone()) {
        Ok(_) => Ok(new_state),
        Err(e) => Err(e),
    }
}

pub fn initialize() -> Result<State, StorageError> {
    match LocalStorage::get::<String>(ONBOARDING_KEY) {
        Ok(_) => {
            // Get existing state
            get_state()
        }
        Err(_) => {
            LocalStorage::set::<String>(ONBOARDING_KEY, String::from(ONBOARDING_KEY))?;
            // Return default state
            set_state(State::default())
        }
    }
}
