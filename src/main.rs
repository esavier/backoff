pub mod backoff;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use rand::{Rng, rng};

    use crate::backoff::clock::{Clock, TokioClock};
    use crate::backoff::policies::capped::Capped;
    use crate::backoff::policies::init::Init;
    use crate::backoff::policies::linear::Linear;
    use crate::backoff::policies::max_retries::MaxReties;
    use crate::backoff::policies::random_jitter::RandomJitter;
    use crate::backoff::policies::*;
    use crate::backoff::{core::BackoffTimer, runnable::Runnable};
    use async_trait::async_trait;

    pub struct SomeFunction {}
    impl SomeFunction {
        pub fn new() -> Self {
            SomeFunction {}
        }
    }

    #[async_trait]
    impl Runnable for SomeFunction {
        async fn run(&self) -> Result<(), String> {
            if rng().random_range(0..100) == 0 {
                Ok(())
            } else {
                Err("Error".to_string())
            }
        }
    }

    pub struct fakeTestClock {}

    #[async_trait]
    impl Clock for fakeTestClock {
        async fn sleep(&self, duration: std::time::Duration) {
            println!(" ---fake sleeping for {} milis", duration.as_millis());
        }
    }

    #[tokio::test]
    async fn it_works() {
        let runnable = SomeFunction::new();
        // let clock = TokioClock::new();
        let clock = fakeTestClock {};
        let mut policy_linear = Linear::new(1000);
        let mut policy_capped = Capped::new(10000);
        let mut policy_max_retries = MaxReties::new(30);
        let mut policy_random_jitter =
            RandomJitter::new(1000, random_jitter::RandomnessDistribution::Gaussian);

        let mut timer = BackoffTimer::new(0, clock, runnable)
            .with_policy(&mut policy_linear)
            .with_policy(&mut policy_max_retries)
            .with_policy(&mut policy_random_jitter)
            .with_policy(&mut policy_capped);

        timer.run_with_backoff().await.unwrap();
    }
}
