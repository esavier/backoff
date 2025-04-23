// use this file as template to implement another policies if necessary

use crate::backoff::{
    backoff::BackoffTimer, clock::Clock, policies::traits::BackoffPolicyResult,
    policies::traits::RetryPolicy, runnable::Runnable,
};

pub struct PolicyStructure {
    // some settings
    value: u64,
}

impl PolicyStructure {
    // extra code if any
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

// policy implementation
impl<C, R> RetryPolicy<C, R> for PolicyStructure
where
    C: Clock,
    R: Runnable,
{
    fn apply(&mut self, backoff: &mut BackoffTimer<C, R>) -> BackoffPolicyResult {
        // the result is a mofidication of the backoff's current_loop_milis
        // setting
        backoff.current_loop_milis = self.randomize(backoff.current_loop_milis);
        BackoffPolicyResult::Continue
    }
}
