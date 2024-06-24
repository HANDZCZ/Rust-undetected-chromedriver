use crate::get_chrome_version::get_chrome_version;

pub async fn fetch_chromedriver(
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;

    let installed_version = get_chrome_version(os).await?;
    let chromedriver_url: String;
    if installed_version.as_str() >= "114" {
        // Fetch the correct version
        let url = "https://googlechromelabs.github.io/chrome-for-testing/latest-versions-per-milestone.json";
        let resp = client.get(url).send().await?;
        let body = resp.bytes().await?;
        let json = serde_json::from_slice::<serde_json::Value>(&body)?;
        let version = json["milestones"][installed_version]["version"]
            .as_str()
            .unwrap();

        // Fetch the chromedriver binary
        chromedriver_url = match os {
            "linux" => format!(
                "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/{}/{}/{}",
                version, "linux64", "chromedriver-linux64.zip"
            ),
            "macos" => format!(
                "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/{}/{}/{}",
                version, "mac-x64", "chromedriver-mac-x64.zip"
            ),
            "windows" => format!(
                "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/{}/{}/{}",
                version, "win64", "chromedriver-win64.zip"
            ),
            _ => panic!("Unsupported OS!"),
        };
    } else {
        let resp = client
            .get(format!(
                "https://chromedriver.storage.googleapis.com/LATEST_RELEASE_{}",
                installed_version
            ))
            .send()
            .await?;
        let body = resp.text().await?;
        chromedriver_url = match os {
            "linux" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
                body
            ),
            "windows" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
                body
            ),
            "macos" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_mac64.zip",
                body
            ),
            _ => panic!("Unsupported OS!"),
        };
    }

    let resp = client.get(&chromedriver_url).send().await?;
    let body = resp.bytes().await?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(body))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.mangled_name();
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            let outpath_relative = outpath.file_name().unwrap();
            let mut outfile = std::fs::File::create(outpath_relative)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
