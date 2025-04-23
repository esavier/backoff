use crate::backoff::{
    clock::Clock, core::BackoffTimer, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct MaxReties {
    // some settings
    max_retries: u64,
}

impl MaxReties {
    // extra code if any
    pub fn new(max_retries: u64) -> Self {
        Self { max_retries }
    }
}

// we simply want to check if we even should retry the request
impl<C, R> RetryPolicy<C, R> for MaxReties
where
    C: Clock,
    R: Runnable,
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult {
        // the result is a mofidication of the backoff's current_loop_milis
        // setting
        match backoff.counter {
            x if x >= self.max_retries => BackoffPolicyResult::Stop,
            _ => BackoffPolicyResult::Continue,
        }
    }
}
