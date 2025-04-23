use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait Clock: Send + Sync {
    async fn sleep(&self, duration: Duration);
}

pub struct TokioClock;

impl TokioClock {
    pub fn new() -> Self {
        TokioClock {}
    }
}

impl Default for TokioClock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Clock for TokioClock {
    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
