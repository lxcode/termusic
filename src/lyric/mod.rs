pub mod lrc;

use anyhow::{anyhow,Result};
use serde::Deserialize;
use std::fmt;
use std::path::Path;

pub struct SongTag {
    artist: Vec<String>,
    title: Option<String>,
    album: Option<String>,
    lang_ext: Option<String>,
    service_provider: Option<String>,
    song_id: Option<String>,
    lyric_id: Option<String>,
}

// TagNetease is the tag get from netease
#[derive(Deserialize)]
struct TagNetease {
    album: String,
    artist: Vec<String>,
    id: i64,
    lyric_id: i64,
    name: String,
    pic_id: String,
    source: String,
    url_id: i64,
}

// TagKugou is the tag get from kugou
#[derive(Deserialize)]
struct TagKugou {
    album: String,
    artist: Vec<String>,
    id: String,
    lyric_id: String,
    name: String,
    pic_id: String,
    source: String,
    url_id: String,
}

// TagLyric is the lyric json get from both netease and kugou
struct TagLyric {
    lyric: String,
    tlyric: String,
}

pub fn lyric_options(search: &str) -> Result<Vec<SongTag>> {
    let service_provider = "netease";
    let mut results = get_lyric_options(search, service_provider)?;
    let service_provider = "kugou";
    let results2 = get_lyric_options(search, service_provider)?;

    results.extend(results2);

    Ok(results)
}

pub(super) fn get_lyric_options(search: &str, service_provider: &str) -> Result<Vec<SongTag>> {
    let p: &Path = Path::new(search);
    let search = p.file_stem().unwrap().to_str().unwrap();
    let url_search = "http://api.sunyj.xyz/?";
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url_search)
        .query(&[("site", service_provider), ("search", search)])
        .send()?;

    if resp.status()!=200 {
        return Err(anyhow!("Network error?"))
    }

    // println!("{:?}", resp);
    let mut result_tags: Vec<SongTag> = vec![];

    match service_provider {
        "kugou" => {
            let tag_kugou: Vec<TagKugou> = resp.json::<Vec<TagKugou>>()?;
            for v in tag_kugou.iter() {
                let song_tag: SongTag = SongTag {
                    artist: v.artist.clone(),
                    title: Some(v.name.clone()),
                    album: Some(v.album.clone()),
                    lang_ext: Some(String::from("chi")),
                    service_provider: Some(String::from("kugou")),
                    song_id: Some(v.id.clone()),
                    lyric_id: Some(v.lyric_id.clone()),
                };
                result_tags.push(song_tag);
            }
        }
        "netease" => {
            let tag_netease: Vec<TagNetease> = resp.json::<Vec<TagNetease>>()?;
            for v in tag_netease.iter() {
                let song_tag: SongTag = SongTag {
                    artist: v.artist.clone(),
                    title: Some(v.name.clone()),
                    album: Some(v.album.clone()),
                    lang_ext: Some(String::from("chi")),
                    service_provider: Some(String::from("kugou")),
                    song_id: Some(format!("{}", v.id)),
                    lyric_id: Some(format!("{}", v.lyric_id)),
                };
                result_tags.push(song_tag);
            }
        }
        &_ => {}
    }

    Ok(result_tags)
}
pub fn fetch_lyric(song_tag: &SongTag) -> Result<String> {
	// urlSearch := "http://api.sunyj.xyz"
    let url_search = "http://api.sunyj.xyz/?";
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url_search)
        .query(&[("site", &song_tag.service_provider), ("lyric", &song_tag.lyric_id)])
        .send()?;

    if resp.status()!=200 {
        return Err(anyhow!("Network error?"))
    }

	// var tagLyric tagLyric
	// err = json.NewDecoder(resp.Body).Decode(&tagLyric)
	// if err != nil {
	// 	return "", tracerr.Wrap(err)
	// }
	// lyricString = tagLyric.Lyric
	// if lyricString == "" {
	// 	return "", errors.New("no lyric available")
	// }

	// if looksLikeLRC(lyricString) {
	// 	lyricString = cleanLRC(lyricString)
	// 	return lyricString, nil
	// }
	// return "", errors.New("lyric not compatible")
    Ok(String::from("abc"))
}


impl fmt::Display for SongTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}-{}", self.file, self.file,)

        let mut artists: String = String::from("");

        for a in self.artist.iter() {
            artists += a;
        }

        let title = self.title.clone().unwrap_or(String::from("Unknown Title"));
        let album = self.album.clone().unwrap_or(String::from("Unknown Album"));

        write!(f, "{:.12}《{:.12}》{:.10}", artists, title, album,)
    }
}
