pub use capabilities::DefaultCapabilitiesBuilder;
use fetch_chromedriver::fetch_chromedriver;
use patch_chromedriver::patch_chromedriver;
use rand::Rng;
use spawn_chromedriver::spawn_chromedriver;
use std::{error::Error, fmt::Display, process::Child};
pub use thirtyfour;
use thirtyfour::WebDriver;
mod capabilities;
mod driver_ext;
mod fetch_chromedriver;
mod get_chrome_version;
mod patch_chromedriver;
mod spawn_chromedriver;
pub use driver_ext::Chrome;

pub const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

/// Fetches a new ChromeDriver executable and patches it to prevent detection.
/// Returns a WebDriver instance (with default capabilities) and handle to chromedriver process.
pub async fn chrome() -> Result<(WebDriver, Child), Box<dyn std::error::Error + Send + Sync>> {
    chrome_with_capabilities(DefaultCapabilitiesBuilder::new().into_chrome_caps()).await
}

/// Fetches a new ChromeDriver executable and patches it to prevent detection.
/// Returns a WebDriver instance and handle to chromedriver process.
pub async fn chrome_with_capabilities(
    capabilities: thirtyfour::ChromeCapabilities,
) -> Result<(WebDriver, Child), Box<dyn std::error::Error + Send + Sync>> {
    let os = std::env::consts::OS;
    if std::path::Path::new("chromedriver").exists()
        || std::path::Path::new("chromedriver.exe").exists()
    {
        tracing::info!("ChromeDriver already exists!");
    } else {
        tracing::info!("ChromeDriver does not exist! Fetching...");
        fetch_chromedriver().await?;
    }
    let chromedriver_executable = match os {
        "linux" => "chromedriver_PATCHED",
        "macos" => "chromedriver_PATCHED",
        "windows" => "chromedriver_PATCHED.exe",
        _ => return Err(UnsupportedOS.into()),
    };
    if std::path::Path::new(chromedriver_executable).exists() {
        tracing::info!("Detected patched chromedriver executable!");
    } else {
        patch_chromedriver(chromedriver_executable)?;
    }
    tracing::info!("Starting chromedriver...");
    let port: u16 = rand::thread_rng().gen_range(2000..5000);
    let chrome_driver_handle = spawn_chromedriver(chromedriver_executable, port)?;
    let mut driver = None;
    let mut attempt = 0u8;
    while driver.is_none() && attempt < 20 {
        attempt += 1;
        match WebDriver::new(&format!("http://127.0.0.1:{}", port), capabilities.clone()).await {
            Ok(d) => driver = Some(d),
            Err(_) => tokio::time::sleep(std::time::Duration::from_millis(250)).await,
        }
    }
    let driver = driver.ok_or(DriverCreationFailed)?;
    Ok((driver, chrome_driver_handle))
}

#[derive(Debug)]
struct DriverCreationFailed;

impl Display for DriverCreationFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Driver creation failed.")
    }
}
impl Error for DriverCreationFailed {}

#[derive(Debug)]
struct UnsupportedOS;

impl Display for UnsupportedOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Your OS is not supported.")
    }
}
impl Error for UnsupportedOS {}
