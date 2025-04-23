use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct Linear {
    step: u64,
}

impl Linear {
    // extra code if any
    pub fn new(step: u64) -> Self {
        Self { step }
    }
}

// this will set (not modify!) the current loop milis
// to the ammount based linearly on counter and internal step
// - will overwrite the value
// - can be used instead of `init`
impl<C, R> RetryPolicy<C, R> for Linear
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        println!(
            "   Policy::Linear: Counter: {} * {} = {}",
            backoff.counter,
            self.step,
            backoff.counter * self.step
        );
        BackoffPolicyResult::SetMilis(backoff.counter * self.step)
    }
}
