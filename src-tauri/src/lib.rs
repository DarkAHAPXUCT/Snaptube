use serde_json::Value;
use regex::Regex;

#[tauri::command]
async fn fetch_video_info(video_url: String) -> Result<String, String> {
    let re = Regex::new(r"(?:v=|/)([0-9A-Za-z_-]{11})").map_err(|e| e.to_string())?;
    let video_id = re.captures(&video_url)
        .map(|caps| caps.get(1).unwrap().as_str().to_string())
        .ok_or("Invalid URL")?;

    let servers = [
        "https://pipedapi.kavin.rocks",
        "https://api.piped.private.coffee",
        "https://pipedapi.adminforge.de"
    ];
    let client = reqwest::Client::new();
    for server in servers {
        let url = format!("{}/streams/{}", server, video_id);
        if let Ok(res) = client.get(&url).send().await {
            if res.status().is_success() {
                if let Ok(json) = res.json::<Value>().await {
                    if let Some(streams) = json["audioStreams"].as_array() {
                        if let Some(best) = streams.first() {
                            if let Some(url) = best["url"].as_str() {
                                return Ok(url.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    Err("All servers failed".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_video_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
