use crate::player::{PlayerTrait, Status};
use crate::track::Track;
// use crate::souvlaki::{
//     MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig,
// };
use crate::ui::model::Model;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};
// use std::str::FromStr;
use std::sync::mpsc::{self, Receiver};
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread::{self, JoinHandle};

pub struct Mpris {
    controls: MediaControls,
    pub rx: Receiver<MediaControlEvent>,
}
impl Default for Mpris {
    fn default() -> Self {
        // #[cfg(not(target_os = "windows"))]
        let hwnd = None;

        // #[cfg(target_os = "windows")]
        // let hwnd = {
        //     use raw_window_handle::windows::WindowsHandle;

        //     let handle: WindowsHandle = unimplemented!();
        //     Some(handle.hwnd)
        // };

        let config = PlatformConfig {
            dbus_name: "termusic",
            display_name: "Termuisc in Rust",
            hwnd,
        };

        let mut controls = MediaControls::new(config).unwrap();

        let (tx, rx) = mpsc::sync_channel(32);
        // The closure must be Send and have a static lifetime.
        controls
            .attach(move |event: MediaControlEvent| {
                tx.send(event).ok();
            })
            .ok();

        Self { controls, rx }
    }
}

impl Mpris {
    pub fn add_and_play(&mut self, song_str: &str) {
        if let Ok(track) = Track::read_from_path(song_str) {
            self.controls
                .set_metadata(MediaMetadata {
                    title: Some(track.title().unwrap_or("Unknown Title")),
                    artist: Some(track.artist().unwrap_or("Unknown Artist")),
                    album: Some(track.album().unwrap_or("")),
                    ..MediaMetadata::default()
                })
                .ok();
        }
        self.controls
            .set_playback(MediaPlayback::Playing { progress: None })
            .ok();
    }
    pub fn pause(&mut self) {
        self.controls
            .set_playback(MediaPlayback::Paused { progress: None })
            .ok();
    }
    pub fn resume(&mut self) {
        self.controls
            .set_playback(MediaPlayback::Playing { progress: None })
            .ok();
    }
}

impl Model {
    pub fn mpris_handler(&mut self, e: MediaControlEvent) {
        match e {
            MediaControlEvent::Next => {
                self.player.skip();
            }
            MediaControlEvent::Previous => {
                self.player_previous();
            }
            MediaControlEvent::Pause => {
                self.player.pause();
                self.progress_update_title();
            }
            MediaControlEvent::Toggle => {
                if self.player.is_paused() {
                    // self.player.status = Status::Running;
                    self.player.set_status(Status::Running);
                    self.player.resume();
                } else {
                    // self.player.status = Status::Paused;
                    self.player.set_status(Status::Paused);
                    self.player.pause();
                }
                self.progress_update_title();
            }
            MediaControlEvent::Play => {
                self.player.resume();
                self.progress_update_title();
            }
            // MediaControlEvent::Seek(x) => match x {
            //     SeekDirection::Forward => activity.player.seek(5).ok(),
            //     SeekDirection::Backward => activity.player.seek(-5).ok(),
            // },
            // MediaControlEvent::SetPosition(position) => {
            //     let _position = position. / 1000;
            // }
            MediaControlEvent::OpenUri(uri) => {
                self.player.add_and_play(&uri);
            }
            _ => {}
        }
    }

    pub fn update_mpris(&mut self) {
        if let Ok(m) = self.mpris.rx.try_recv() {
            self.mpris_handler(m);
        }
    }
}
