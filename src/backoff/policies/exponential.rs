use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct Exponential {
    step: u64,
}

impl Exponential {
    // extra code if any
    pub fn new(step: u64) -> Self {
        Self { step }
    }
}

// this will set (not modify!) the current loop milis
// to the ammount based exponentially on step and counter
// - will overwrite the value
// - can be used instead of `init`
impl<C, R> RetryPolicy<C, R> for Exponential
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        BackoffPolicyResult::SetMilis(self.step * backoff.counter.pow(2))
    }
}
