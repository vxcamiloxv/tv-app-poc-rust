pub mod model;

use model::{ResponseData, VideoPicture};

const API_URL: &str = "https://api.pexels.com/videos/search";
const API_KEY: &str = "563492ad6f917000010000013ff3f8d8531a403283fb59de554c34d6";

pub async fn get_videos(category: String) -> Result<ResponseData, reqwest::Error> {
    let url = format!("{api_url}?query={query}", api_url = API_URL, query = category);
    let response = reqwest::Client::new()
        .get(&url)
        .header("Authorization", API_KEY)
        .send()
        .await?;
    let data = response.json().await?;

    Ok(data)
}

pub fn get_video_images() -> Result<Vec<VideoPicture>, reqwest::Error> {
    Ok(vec![
        VideoPicture {
            picture: "https://static-videos.pexels.com/videos/2499611/pictures/preview-0.jpg".to_string(),
            nr: 0
        },
        VideoPicture {
            picture: "https://static-videos.pexels.com/videos/2499611/pictures/preview-1.jpg".to_string(),
            nr: 1
        },
        VideoPicture {
            picture: "https://static-videos.pexels.com/videos/2499611/pictures/preview-2.jpg".to_string(),
            nr: 2
        },
        VideoPicture {
            picture: "https://static-videos.pexels.com/videos/2499611/pictures/preview-3.jpg".to_string(),
            nr: 3
        }
    ])
}
