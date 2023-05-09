use serde::{Deserialize, Serialize};

/// A struct for pixel API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
  pub videos: Vec<Video>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
  pub video_files: Vec<VideoFile>,
  pub video_pictures: Vec<VideoPicture>,
  pub user: User,
  pub duration: u32,
  pub image: String,
  pub height: u32,
  pub width: u32,
  pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFile {
  pub link: String,
  pub quality: String,
  pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoPicture {
  pub picture: String,
  pub nr: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub name: String,
}
