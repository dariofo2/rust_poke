use std::time::Duration;

use tokio::time::sleep;

pub struct AsyncUtils;

impl AsyncUtils {
    pub async fn waitForSecs () -> u32 {
        sleep(Duration::from_millis(5000)).await;
        return 5;
    }
}