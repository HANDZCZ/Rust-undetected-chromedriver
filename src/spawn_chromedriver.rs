#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::fs::PermissionsExt;
use std::process::{Child, Command, Stdio};

pub fn spawn_chromedriver(chromedriver_executable: &str, port: u16) -> Child {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let mut perms = std::fs::metadata(chromedriver_executable)
            .unwrap()
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(chromedriver_executable, perms).unwrap();
    }
    let mut chrome_driver_handle = Command::new(format!("./{}", chromedriver_executable))
        .stdout(Stdio::piped())
        .arg(format!("--port={}", port))
        .spawn()
        .expect("Failed to start chromedriver!");
    let chrome_driver_stdout = chrome_driver_handle.stdout.take().unwrap();
    std::thread::Builder::new()
        .name("ChromeDriverThread".to_string())
        .spawn(|| {
            use std::io::BufRead;
            let lines = std::io::BufReader::new(chrome_driver_stdout).lines();
            for line in lines {
                tracing::info!("{}", line.unwrap());
            }
        })
        .unwrap();
    chrome_driver_handle
}
