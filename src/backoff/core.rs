use crate::backoff::runnable::Runnable;
use crate::backoff::{
    clock::Clock, policies::traits::BackoffPolicyResult, policies::traits::RetryPolicy,
};
use std::time::Duration;
pub struct BackoffTimer<'timer, C: Clock, R: Runnable> {
    pub counter: u64,
    pub start: u64,
    pub current_loop_milis: u64,

    clock: C,
    runnable: R,
    // order of execution matters here
    // policies are applied in the order they are added
    policies: Vec<&'timer mut dyn RetryPolicy<C, R>>,
}

impl<'timer, C: Clock, R: Runnable> BackoffTimer<'timer, C, R> {
    pub fn new(milis: u64, clock: C, runnable: R) -> Self {
        Self {
            counter: 0,
            start: milis,
            current_loop_milis: milis,
            clock,
            runnable,
            policies: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy: &'timer mut dyn RetryPolicy<C, R>) {
        self.policies.push(policy);
    }

    pub async fn run_with_backoff(&mut self) -> Result<(), String> {
        loop {
            // by default we are just adding exponential backoff
            // if policies exists we are using those instead
            match self.policies.len() {
                0 => {
                    self.current_loop_milis = self.counter.pow(2) * self.start;
                }
                _ => {
                    for each in self.policies.iter() {
                        match each.apply(self) {
                            BackoffPolicyResult::Stop => return Ok(()),
                            BackoffPolicyResult::Continue => (),
                            BackoffPolicyResult::SetMilis(milis) => {
                                self.current_loop_milis = milis;
                            }
                            BackoffPolicyResult::SetCounter(counter) => {
                                self.counter = counter;
                            }
                        }
                    }
                }
            };

            match self.runnable.run().await {
                Ok(_) => {
                    println!("!! Run Success !!");
                    println!("   Counter: {}", self.counter);
                    println!("   Current Loop Milis: {}", self.current_loop_milis);
                    return Ok(());
                }
                Err(e) => {
                    println!("!! Run Failed !!");
                    println!("   Error: {}", e);
                    println!("   Counter: {}", self.counter);
                    println!("   Current Loop Milis: {}", self.current_loop_milis);
                    println!("   ...looping");
                    self.counter += 1;
                    // tokio::time::sleep(Duration::from_millis(self.current_loop_milis)).await;
                    self.clock
                        .sleep(Duration::from_millis(self.current_loop_milis))
                        .await;
                }
            }
        }
    }

    // bbuilder pattern for adding policies
    pub fn with_policy(mut self, policy: &'timer mut dyn RetryPolicy<C, R>) -> Self {
        self.add_policy(policy);
        self
    }
}
