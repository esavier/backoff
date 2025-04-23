use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct Capped {
    // some settings
    max: u64,
}

impl Capped {
    // extra code if any
    pub fn new(duration_cap: u64) -> Self {
        Self { max: duration_cap }
    }
}

// policy implementation
impl<C, R> RetryPolicy<C, R> for Capped
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        match backoff.current_loop_milis {
            x if x > self.max => BackoffPolicyResult::SetMilis(self.max),
            _ => BackoffPolicyResult::Continue,
        }
    }
}
