use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct Init {}

impl Init {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Init {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, R> RetryPolicy<C, R> for Init
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        let new_milis: u64 = backoff.start;
        BackoffPolicyResult::SetMilis(new_milis)
    }
}
