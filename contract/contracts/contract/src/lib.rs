#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, Address, Map, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub subscriber: Address,
    pub plan: Symbol,
    pub expires_at: u64,
}

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {

    // Create or renew subscription
    pub fn subscribe(env: Env, user: Address, plan: Symbol, duration: u64) {
        user.require_auth();

        let current_time = env.ledger().timestamp();
        let expires_at = current_time + duration;

        let sub = Subscription {
            subscriber: user.clone(),
            plan,
            expires_at,
        };

        env.storage().instance().set(&user, &sub);
    }

    // Check subscription status
    pub fn is_active(env: Env, user: Address) -> bool {
        if let Some(sub) = env.storage().instance().get::<Address, Subscription>(&user) {
            let current_time = env.ledger().timestamp();
            return current_time < sub.expires_at;
        }
        false
    }

    // Get subscription details
    pub fn get_subscription(env: Env, user: Address) -> Option<Subscription> {
        env.storage().instance().get(&user)
    }

    // Cancel subscription
    pub fn cancel(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().remove(&user);
    }
}