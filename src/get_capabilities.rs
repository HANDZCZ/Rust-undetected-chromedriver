use crate::USER_AGENT;
use thirtyfour::{ChromeCapabilities, DesiredCapabilities};

pub fn get_capabilities() -> ChromeCapabilities {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_no_sandbox().unwrap();
    caps.set_disable_dev_shm_usage().unwrap();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")
        .unwrap();
    caps.add_chrome_arg("window-size=1920,1080").unwrap();
    let user_agent = format!("user-agent={USER_AGENT}");
    caps.add_chrome_arg(&user_agent).unwrap();
    caps.add_chrome_arg("disable-infobars").unwrap();
    caps.add_chrome_option("excludeSwitches", ["enable-automation"])
        .unwrap();
    caps
}
