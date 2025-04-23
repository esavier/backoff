use crate::backoff::core::BackoffTimer;

// applicable policy
// modifies the state of the provided backoff structure
// remember that the first policy should always by the `init` policy
pub trait RetryPolicy<C: crate::backoff::clock::Clock, R: crate::backoff::runnable::Runnable>:
    Send + Sync
{
    fn apply(&self, backoff: &BackoffTimer<C, R>) -> BackoffPolicyResult;
}

pub enum BackoffPolicyResult {
    // set counter to provided value and continue
    SetCounter(u64),
    // set milis to provided value and continue
    SetMilis(u64),
    // no changes, continue
    Continue,
    // stop the backoff
    Stop,
}
