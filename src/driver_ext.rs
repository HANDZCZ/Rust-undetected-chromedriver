use std::{error::Error, process::Child, time::Duration};

use thirtyfour::WebDriver;

use crate::chrome;

#[async_trait::async_trait]
pub trait Chrome: Sized {
    async fn new() -> (Self, Child);
    async fn borrow(&self) -> &WebDriver;
    async fn goto(&self, url: &str) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
impl Chrome for WebDriver {
    async fn new() -> (WebDriver, Child) {
        chrome().await.unwrap()
    }

    async fn goto(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let driver = self.borrow().await;
        driver
            .execute(&format!(r#"window.open("{}", "_blank");"#, url), vec![])
            .await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let first_window = driver
            .windows()
            .await?
            .first()
            .expect("Unable to get first windows")
            .clone();

        driver.switch_to_window(first_window).await?;
        driver.close_window().await?;
        let first_window = driver
            .windows()
            .await?
            .last()
            .expect("Unable to get last windows")
            .clone();
        driver.switch_to_window(first_window).await?;
        Ok(())
    }

    async fn borrow(&self) -> &WebDriver {
        self
    }
}
