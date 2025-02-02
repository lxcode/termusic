use crate::{
    config::{Keys, Settings},
    player::Loop,
    track::Track,
    ui::{GSMsg, Id, Model, Msg, PLMsg},
};

use crate::player::PlayerTrait;
use crate::sqlite::TrackForDB;
use crate::utils::{filetype_supported, is_playlist};
use anyhow::{anyhow, bail, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, PropPayload, PropValue, TableBuilder, TextSpan};
use tuirealm::{
    event::{Key, KeyEvent, NoUserEvent},
    AttrValue, Attribute, Component, Event, MockComponent, State, StateValue,
};

use tuirealm::props::{Borders, Color};

#[derive(MockComponent)]
pub struct Playlist {
    component: Table,
    keys: Keys,
}

impl Playlist {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default().modifiers(BorderType::Rounded).color(
                        config
                            .style_color_symbol
                            .playlist_border()
                            .unwrap_or(Color::Blue),
                    ),
                )
                .background(
                    config
                        .style_color_symbol
                        .playlist_background()
                        .unwrap_or(Color::Reset),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .playlist_foreground()
                        .unwrap_or(Color::Yellow),
                )
                .title(" Playlist ", Alignment::Left)
                .scroll(true)
                .highlighted_color(
                    config
                        .style_color_symbol
                        .playlist_highlight()
                        .unwrap_or(Color::LightBlue),
                )
                .highlighted_str(&config.style_color_symbol.playlist_highlight_symbol)
                .rewind(false)
                .step(4)
                .row_height(1)
                .headers(&["Duration", "Artist", "Title", "Album"])
                .column_spacing(2)
                .widths(&[12, 20, 25, 43])
                .table(
                    TableBuilder::default()
                        .add_col(TextSpan::from("Empty"))
                        .add_col(TextSpan::from("Empty Queue"))
                        .add_col(TextSpan::from("Empty"))
                        .build(),
                ),
            keys: config.keys.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for Playlist {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _cmd_result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(key) if key == self.keys.global_down.key_event() => {
                self.perform(Cmd::Move(Direction::Down))
            }
            Event::Keyboard(key) if key == self.keys.global_up.key_event() => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(key) if key == self.keys.global_goto_top.key_event() => {
                self.perform(Cmd::GoTo(Position::Begin))
            }
            Event::Keyboard(key) if key == self.keys.global_goto_bottom.key_event() => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::Playlist(PLMsg::TableBlur))
            }
            Event::Keyboard(key) if key == self.keys.playlist_delete.key_event() => {
                match self.component.state() {
                    State::One(StateValue::Usize(index_selected)) => {
                        return Some(Msg::Playlist(PLMsg::Delete(index_selected)))
                    }
                    _ => return Some(Msg::None),
                }
            }
            Event::Keyboard(key) if key == self.keys.playlist_delete_all.key_event() => {
                return Some(Msg::Playlist(PLMsg::DeleteAll))
            }
            Event::Keyboard(key) if key == self.keys.playlist_shuffle.key_event() => {
                return Some(Msg::Playlist(PLMsg::Shuffle))
            }
            Event::Keyboard(key) if key == self.keys.playlist_mode_cycle.key_event() => {
                return Some(Msg::Playlist(PLMsg::LoopModeCycle))
            }
            Event::Keyboard(key) if key == self.keys.playlist_play_selected.key_event() => {
                if let State::One(StateValue::Usize(index)) = self.state() {
                    return Some(Msg::Playlist(PLMsg::PlaySelected(index)));
                }
                CmdResult::None
            }
            Event::Keyboard(key) if key == self.keys.playlist_add_front.key_event() => {
                return Some(Msg::Playlist(PLMsg::AddFront))
            }
            Event::Keyboard(key) if key == self.keys.playlist_search.key_event() => {
                return Some(Msg::GeneralSearch(GSMsg::PopupShowPlaylist))
            }
            Event::Keyboard(key) if key == self.keys.playlist_swap_down.key_event() => {
                match self.component.state() {
                    State::One(StateValue::Usize(index_selected)) => {
                        self.perform(Cmd::Move(Direction::Down));
                        return Some(Msg::Playlist(PLMsg::SwapDown(index_selected)));
                    }
                    _ => return Some(Msg::None),
                }
            }
            Event::Keyboard(key) if key == self.keys.playlist_swap_up.key_event() => {
                match self.component.state() {
                    State::One(StateValue::Usize(index_selected)) => {
                        self.perform(Cmd::Move(Direction::Up));
                        return Some(Msg::Playlist(PLMsg::SwapUp(index_selected)));
                    }
                    _ => return Some(Msg::None),
                }
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

impl Model {
    pub fn playlist_reload(&mut self) {
        assert!(self
            .app
            .remount(
                Id::Playlist,
                Box::new(Playlist::new(&self.config)),
                Vec::new()
            )
            .is_ok());
        self.playlist_sync();
    }

    fn playlist_add_playlist(&mut self, current_node: &str) -> Result<()> {
        let p = Path::new(current_node);
        let p_base = p.parent().ok_or_else(|| anyhow!("cannot find path root"))?;
        let str = std::fs::read_to_string(p)?;
        let items =
            crate::playlist::decode(&str).map_err(|e| anyhow!("playlist decode error: {}", e))?;
        let mut vec = vec![];
        for item in items {
            if !filetype_supported(&item) {
                continue;
            }
            if let Ok(pathbuf) = Self::playlist_get_absolute_pathbuf(&item, p_base) {
                vec.push(pathbuf.to_string_lossy().to_string());
            }
        }
        self.playlist_add_items_common(&vec);
        self.playlist_sync();
        Ok(())
    }

    fn playlist_get_absolute_pathbuf(item: &str, p_base: &Path) -> Result<PathBuf> {
        let url_decoded = urlencoding::decode(item)?.into_owned();
        let mut url = url_decoded.clone();
        let mut pathbuf = PathBuf::from(p_base);
        if url_decoded.starts_with("http") {
            bail!("http not supported");
        }
        if url_decoded.starts_with("file") {
            url = url_decoded.replace("file://", "");
        }
        if Path::new(&url).is_relative() {
            pathbuf.push(url);
        } else {
            pathbuf = PathBuf::from(url);
        }
        Ok(pathbuf)
    }

    fn playlist_add_item(&mut self, current_node: &str, add_playlist_front: bool) -> Result<()> {
        if is_playlist(current_node) {
            self.playlist_add_playlist(current_node)?;
            return Ok(());
        }
        if !filetype_supported(current_node) {
            return Ok(());
        }
        let item = Track::read_from_path(current_node)?;
        if add_playlist_front {
            self.player.playlist.tracks.push_front(item);
        } else {
            self.player.playlist.tracks.push_back(item);
        }
        self.playlist_sync();
        Ok(())
    }

    pub fn playlist_add(&mut self, current_node: &str) {
        let p: &Path = Path::new(&current_node);
        if !p.exists() {
            return;
        }

        if p.is_dir() {
            self.playlist_add_all_from_treeview(p);
        } else if let Err(e) = self.playlist_add_item(current_node, self.config.add_playlist_front)
        {
            self.mount_error_popup(format!("Add Playlist error: {}", e).as_str());
        }
    }

    fn playlist_add_items_common(&mut self, vec: &[String]) {
        let mut index = 0;
        for s in vec {
            if !filetype_supported(s) {
                continue;
            }
            if self.config.add_playlist_front {
                if let Ok(item) = Track::read_from_path(s) {
                    self.player.playlist.tracks.insert(index, item);
                    index += 1;
                }
                continue;
            }

            self.playlist_add_item(s, false).ok();
        }
        self.playlist_sync();
    }

    fn playlist_add_all_from_treeview(&mut self, p: &Path) {
        let new_items = Self::library_dir_children(p);
        self.playlist_add_items_common(&new_items);
    }

    pub fn playlist_add_all_from_db(&mut self, vec: &[TrackForDB]) {
        let vec2: Vec<String> = vec.iter().map(|f| f.file.clone()).collect();
        self.playlist_add_items_common(&vec2);
    }

    pub fn playlist_sync(&mut self) {
        let mut table: TableBuilder = TableBuilder::default();

        for (idx, record) in self.player.playlist.tracks.iter().enumerate() {
            if idx > 0 {
                table.add_row();
            }

            let duration = record.duration_formatted().to_string();
            let duration_string = format!("[{:^7.7}]", duration);

            let noname_string = "No Name".to_string();
            let name = record.name().unwrap_or(&noname_string);
            let artist = record.artist().unwrap_or(name);
            let title = record.title().unwrap_or("Unknown Title");

            table
                .add_col(TextSpan::new(duration_string.as_str()))
                .add_col(TextSpan::new(artist).fg(tuirealm::tui::style::Color::LightYellow))
                .add_col(TextSpan::new(title).bold())
                .add_col(TextSpan::new(record.album().unwrap_or("Unknown Album")));
        }
        if self.player.playlist.tracks.is_empty() {
            table.add_col(TextSpan::from("0"));
            table.add_col(TextSpan::from("empty playlist"));
            table.add_col(TextSpan::from(""));
            table.add_col(TextSpan::from(""));
        }

        let table = table.build();
        self.app
            .attr(
                &Id::Playlist,
                tuirealm::Attribute::Content,
                tuirealm::AttrValue::Table(table),
            )
            .ok();

        self.playlist_update_title();
    }
    pub fn playlist_delete_item(&mut self, index: usize) {
        if self.player.playlist.is_empty() {
            return;
        }
        self.player.playlist.tracks.remove(index);
        self.playlist_sync();
    }

    pub fn playlist_empty(&mut self) {
        self.player.playlist.tracks.clear();
        self.playlist_sync();
    }

    pub fn playlist_shuffle(&mut self) {
        let mut rng = thread_rng();
        self.player
            .playlist
            .tracks
            .make_contiguous()
            .shuffle(&mut rng);
        self.playlist_sync();
    }

    pub fn playlist_update_library_delete(&mut self) {
        self.player
            .playlist
            .tracks
            .retain(|x| x.file().map_or(false, |p| Path::new(p).exists()));

        self.playlist_sync();
    }

    pub fn playlist_update_title(&mut self) {
        let mut duration = Duration::from_secs(0);
        for v in &self.player.playlist.tracks {
            duration += v.duration();
        }
        let add_queue = if self.config.add_playlist_front {
            if self.config.playlist_display_symbol {
                // "\u{1f51d}"
                "\u{fb22}"
                // "ﬢ"
            } else {
                "next"
            }
        } else if self.config.playlist_display_symbol {
            "\u{fb20}"
            // "ﬠ"
        } else {
            "last"
        };
        let title = format!(
            "\u{2500} Playlist \u{2500}\u{2500}\u{2524} Total {} tracks | {} | Mode: {} | Add to: {} \u{251c}\u{2500}",
            self.player.playlist.len(),
            Track::duration_formatted_short(&duration),
            self.config.loop_mode.display(self.config.playlist_display_symbol),
            add_queue
        );
        self.app
            .attr(
                &Id::Playlist,
                tuirealm::Attribute::Title,
                tuirealm::AttrValue::Title((title, Alignment::Left)),
            )
            .ok();
    }
    pub fn playlist_cycle_loop_mode(&mut self) {
        match self.config.loop_mode {
            Loop::Queue => {
                self.config.loop_mode = Loop::Playlist;
            }
            Loop::Playlist => {
                self.config.loop_mode = Loop::Single;
                if let Some(song) = self.player.playlist.tracks.pop_back() {
                    self.player.playlist.tracks.push_front(song);
                }
            }
            Loop::Single => {
                self.config.loop_mode = Loop::Queue;
                if let Some(song) = self.player.playlist.tracks.pop_front() {
                    self.player.playlist.tracks.push_back(song);
                }
            }
        };
        self.player.config.loop_mode = self.config.loop_mode;
        self.playlist_sync();
        self.playlist_update_title();
    }
    pub fn playlist_play_selected(&mut self, index: usize) {
        if let Some(song) = self.player.playlist.tracks.remove(index) {
            self.player.playlist.tracks.push_front(song);
            self.playlist_sync();
            self.player.stop();
            // self.status = Some(Status::Stopped);
            // self.player_next();
        }
    }

    pub fn playlist_update_search(&mut self, input: &str) {
        let mut table: TableBuilder = TableBuilder::default();
        let mut idx = 0;
        let search = format!("*{}*", input.to_lowercase());
        for record in &self.player.playlist.tracks {
            let artist = record.artist().unwrap_or("Unknown artist");
            let title = record.title().unwrap_or("Unknown title");
            if wildmatch::WildMatch::new(&search).matches(&artist.to_lowercase())
                | wildmatch::WildMatch::new(&search).matches(&title.to_lowercase())
            {
                if idx > 0 {
                    table.add_row();
                }

                let duration = record.duration_formatted().to_string();
                let duration_string = format!("[{:^6.6}]", duration);

                let noname_string = "No Name".to_string();
                let name = record.name().unwrap_or(&noname_string);
                let artist = record.artist().unwrap_or(name);
                let title = record.title().unwrap_or("Unknown Title");
                let file_name = record.file().unwrap_or("no file");

                table
                    .add_col(TextSpan::new(duration_string.as_str()))
                    .add_col(TextSpan::new(artist).fg(tuirealm::tui::style::Color::LightYellow))
                    .add_col(TextSpan::new(title).bold())
                    .add_col(TextSpan::new(file_name));
                // .add_col(TextSpan::new(record.album().unwrap_or("Unknown Album")));
                idx += 1;
            }
        }
        if self.player.playlist.is_empty() {
            table.add_col(TextSpan::from("0"));
            table.add_col(TextSpan::from("empty playlist"));
            table.add_col(TextSpan::from(""));
        }
        let table = table.build();

        self.general_search_update_show(table);
    }

    pub fn playlist_locate(&mut self, index: usize) {
        assert!(self
            .app
            .attr(
                &Id::Playlist,
                Attribute::Value,
                AttrValue::Payload(PropPayload::One(PropValue::Usize(index))),
            )
            .is_ok());
    }
}
