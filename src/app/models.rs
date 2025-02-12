pub use super::gtypes::*;
use std::convert::From;

impl From<&AlbumDescription> for AlbumModel {
    fn from(album: &AlbumDescription) -> Self {
        AlbumModel::new(&album.artist, &album.title, &album.art, &album.id)
    }
}

impl From<AlbumDescription> for AlbumModel {
    fn from(album: AlbumDescription) -> Self {
        Self::from(&album)
    }
}

#[derive(Clone, Debug)]
pub struct AlbumDescription {
    pub title: String,
    pub artist: String,
    pub artist_id: String,
    pub uri: String,
    pub art: String,
    pub songs: Vec<SongDescription>,
    pub id: String,
}

impl PartialEq for AlbumDescription {
    fn eq(&self, other: &AlbumDescription) -> bool {
        self.id == other.id
    }
}

impl Eq for AlbumDescription {}

#[derive(Clone, Debug)]
pub struct SongDescription {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub duration: u32,
    pub art: Option<String>,
}

impl SongDescription {
    pub fn new(title: &str, artist: &str, uri: &str, duration: u32, art: Option<String>) -> Self {
        Self {
            title: title.to_string(),
            artist: artist.to_string(),
            uri: uri.to_string(),
            duration,
            art,
        }
    }
}
#[derive(Clone, Debug)]
pub struct ArtistDescription {
    pub name: String,
    pub albums: Vec<AlbumDescription>,
}
