//! ## Utils
//!
//! `Utils` implements utilities functions to work with layouts

/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// pub mod activity;
// mod activity;
pub mod components;
pub mod model;

use crate::config::ColorTermusic;
use crate::config::Settings;
#[cfg(not(any(feature = "mpv", feature = "gst")))]
use crate::player::PlayerTrait;
use crate::songtag::SongTag;
use model::Model;
use std::time::Duration;
use tuirealm::application::PollStrategy;
use tuirealm::{Application, Update};
// -- internal

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const FORCED_REDRAW_INTERVAL: Duration = Duration::from_millis(1000);

// Let's define the messages handled by our app. NOTE: it must derive `PartialEq`
#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    // AppClose,
    ColorEditor(CEMsg),
    DataBase(DBMsg),
    DeleteConfirmCloseCancel,
    DeleteConfirmCloseOk,
    DeleteConfirmShow,
    ErrorPopupClose,
    GeneralSearch(GSMsg),
    HelpPopupShow,
    HelpPopupClose,
    KeyEditor(KEMsg),
    LayoutTreeView,
    LayoutDataBase,
    Library(LIMsg),
    LyricCycle,
    LyricAdjustDelay(i64),
    PlayerToggleGapless,
    PlayerTogglePause,
    PlayerVolumeUp,
    PlayerVolumeDown,
    PlayerSpeedUp,
    PlayerSpeedDown,
    PlayerSeek(isize),
    Playlist(PLMsg),
    QuitPopupCloseCancel,
    QuitPopupCloseOk,
    QuitPopupShow,
    TagEditor(TEMsg),
    UpdatePhoto,
    YoutubeSearch(YSMsg),
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LIMsg {
    TreeExtendDir(String),
    TreeGoToUpperDir,
    TreeBlur,
    Yank,
    Paste,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DBMsg {
    AddAllToPlaylist,
    AddPlaylist(usize),
    CriteriaBlurDown,
    CriteriaBlurUp,
    SearchResult(usize),
    SearchResultBlurDown,
    SearchResultBlurUp,
    SearchTrack(usize),
    SearchTracksBlurDown,
    SearchTracksBlurUp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PLMsg {
    AddFront,
    NextSong,
    PrevSong,
    TableBlur,
    Add(String),
    Delete(usize),
    DeleteAll,
    LoopModeCycle,
    PlaySelected(usize),
    Shuffle,
    SwapDown(usize),
    SwapUp(usize),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GSMsg {
    PopupShowDatabase,
    PopupShowLibrary,
    PopupShowPlaylist,
    PopupCloseCancel,
    InputBlur,
    PopupUpdateDatabase(String),
    PopupUpdateLibrary(String),
    PopupUpdatePlaylist(String),
    TableBlur,
    PopupCloseDatabaseAddPlaylist,
    PopupCloseLibraryAddPlaylist,
    PopupCloseOkLibraryLocate,
    PopupClosePlaylistPlaySelected,
    PopupCloseOkPlaylistLocate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum YSMsg {
    InputPopupShow,
    InputPopupCloseCancel,
    InputPopupCloseOk(String),
    TablePopupNext,
    TablePopupPrevious,
    TablePopupCloseCancel,
    TablePopupCloseOk(usize),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TEMsg {
    TagEditorRun(String),
    TagEditorClose(Option<String>),
    TECounterDeleteBlurDown,
    TECounterDeleteBlurUp,
    TECounterDeleteOk,
    TEDownload(usize),
    TEEmbed(usize),
    TEHelpPopupShow,
    TEHelpPopupClose,
    TEInputArtistBlurDown,
    TEInputArtistBlurUp,
    TEInputTitleBlurDown,
    TEInputTitleBlurUp,
    TERadioTagBlurDown,
    TERadioTagBlurUp,
    TERadioTagOk,
    TESearch,
    TESelectLyricBlurDown,
    TESelectLyricBlurUp,
    TESelectLyricOk(usize),
    TETableLyricOptionsBlurDown,
    TETableLyricOptionsBlurUp,
    TETextareaLyricBlurDown,
    TETextareaLyricBlurUp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CEMsg {
    ColorChanged(IdColorEditor, ColorTermusic),
    SymbolChanged(IdColorEditor, String),
    ColorEditorShow,
    ColorEditorCloseCancel,
    ColorEditorCloseOk,
    HelpPopupShow,
    HelpPopupClose,
    ColorEditorOkBlurDown,
    ColorEditorOkBlurUp,
    LibraryForegroundBlurDown,
    LibraryForegroundBlurUp,
    LibraryBackgroundBlurDown,
    LibraryBackgroundBlurUp,
    LibraryBorderBlurDown,
    LibraryBorderBlurUp,
    LibraryHighlightBlurDown,
    LibraryHighlightBlurUp,
    LibraryHighlightSymbolBlurDown,
    LibraryHighlightSymbolBlurUp,
    PlaylistForegroundBlurDown,
    PlaylistForegroundBlurUp,
    PlaylistBackgroundBlurDown,
    PlaylistBackgroundBlurUp,
    PlaylistBorderBlurDown,
    PlaylistBorderBlurUp,
    PlaylistHighlightBlurDown,
    PlaylistHighlightBlurUp,
    PlaylistHighlightSymbolBlurDown,
    PlaylistHighlightSymbolBlurUp,
    ProgressForegroundBlurDown,
    ProgressForegroundBlurUp,
    ProgressBackgroundBlurDown,
    ProgressBackgroundBlurUp,
    ProgressBorderBlurDown,
    ProgressBorderBlurUp,
    LyricForegroundBlurDown,
    LyricForegroundBlurUp,
    LyricBackgroundBlurDown,
    LyricBackgroundBlurUp,
    LyricBorderBlurDown,
    LyricBorderBlurUp,
    ThemeSelectBlurDown,
    ThemeSelectBlurUp,
    ThemeSelectLoad(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum KEMsg {
    DatabaseAddAllBlurDown,
    DatabaseAddAllBlurUp,
    DatabaseAddAllInputBlurDown,
    DatabaseAddAllInputBlurUp,
    GlobalColorEditorBlurDown,
    GlobalColorEditorBlurUp,
    GlobalColorEditorInputBlurDown,
    GlobalColorEditorInputBlurUp,
    GlobalDownBlurDown,
    GlobalDownBlurUp,
    GlobalDownInputBlurDown,
    GlobalDownInputBlurUp,
    GlobalGotoBottomBlurDown,
    GlobalGotoBottomBlurUp,
    GlobalGotoBottomInputBlurDown,
    GlobalGotoBottomInputBlurUp,
    GlobalGotoTopBlurDown,
    GlobalGotoTopBlurUp,
    GlobalGotoTopInputBlurDown,
    GlobalGotoTopInputBlurUp,
    GlobalHelpBlurDown,
    GlobalHelpBlurUp,
    GlobalHelpInputBlurDown,
    GlobalHelpInputBlurUp,
    GlobalKeyEditorBlurDown,
    GlobalKeyEditorBlurUp,
    GlobalKeyEditorInputBlurDown,
    GlobalKeyEditorInputBlurUp,
    GlobalLayoutTreeviewBlurDown,
    GlobalLayoutTreeviewBlurUp,
    GlobalLayoutTreeviewInputBlurDown,
    GlobalLayoutTreeviewInputBlurUp,
    GlobalLayoutDatabaseBlurDown,
    GlobalLayoutDatabaseBlurUp,
    GlobalLayoutDatabaseInputBlurDown,
    GlobalLayoutDatabaseInputBlurUp,
    GlobalLeftBlurDown,
    GlobalLeftBlurUp,
    GlobalLeftInputBlurDown,
    GlobalLeftInputBlurUp,
    GlobalLyricAdjustForwardBlurDown,
    GlobalLyricAdjustForwardBlurUp,
    GlobalLyricAdjustBackwardBlurDown,
    GlobalLyricAdjustBackwardBlurUp,
    GlobalLyricAdjustForwardInputBlurDown,
    GlobalLyricAdjustForwardInputBlurUp,
    GlobalLyricAdjustBackwardInputBlurDown,
    GlobalLyricAdjustBackwardInputBlurUp,
    GlobalLyricCyleBlurDown,
    GlobalLyricCyleBlurUp,
    GlobalLyricCyleInputBlurDown,
    GlobalLyricCyleInputBlurUp,
    GlobalPlayerNextBlurDown,
    GlobalPlayerNextBlurUp,
    GlobalPlayerNextInputBlurDown,
    GlobalPlayerNextInputBlurUp,
    GlobalPlayerPreviousBlurDown,
    GlobalPlayerPreviousBlurUp,
    GlobalPlayerPreviousInputBlurDown,
    GlobalPlayerPreviousInputBlurUp,
    GlobalPlayerSeekForwardBlurDown,
    GlobalPlayerSeekForwardBlurUp,
    GlobalPlayerSeekForwardInputBlurDown,
    GlobalPlayerSeekForwardInputBlurUp,
    GlobalPlayerSeekBackwardBlurDown,
    GlobalPlayerSeekBackwardBlurUp,
    GlobalPlayerSeekBackwardInputBlurDown,
    GlobalPlayerSeekBackwardInputBlurUp,
    GlobalPlayerSpeedUpBlurDown,
    GlobalPlayerSpeedUpBlurUp,
    GlobalPlayerSpeedUpInputBlurDown,
    GlobalPlayerSpeedUpInputBlurUp,
    GlobalPlayerSpeedDownBlurDown,
    GlobalPlayerSpeedDownBlurUp,
    GlobalPlayerSpeedDownInputBlurDown,
    GlobalPlayerSpeedDownInputBlurUp,
    GlobalPlayerToggleGaplessBlurDown,
    GlobalPlayerToggleGaplessBlurUp,
    GlobalPlayerToggleGaplessInputBlurDown,
    GlobalPlayerToggleGaplessInputBlurUp,
    GlobalPlayerTogglePauseBlurDown,
    GlobalPlayerTogglePauseBlurUp,
    GlobalPlayerTogglePauseInputBlurDown,
    GlobalPlayerTogglePauseInputBlurUp,
    GlobalQuitBlurDown,
    GlobalQuitBlurUp,
    GlobalQuitInputBlurDown,
    GlobalQuitInputBlurUp,
    GlobalRightBlurDown,
    GlobalRightBlurUp,
    GlobalRightInputBlurDown,
    GlobalRightInputBlurUp,
    GlobalUpBlurDown,
    GlobalUpBlurUp,
    GlobalUpInputBlurDown,
    GlobalUpInputBlurUp,
    GlobalVolumeDownBlurDown,
    GlobalVolumeDownBlurUp,
    GlobalVolumeDownInputBlurDown,
    GlobalVolumeDownInputBlurUp,
    GlobalVolumeUpBlurDown,
    GlobalVolumeUpBlurUp,
    GlobalVolumeUpInputBlurDown,
    GlobalVolumeUpInputBlurUp,
    HelpPopupShow,
    HelpPopupClose,
    KeyChanged(IdKeyEditor),
    KeyEditorShow,
    KeyEditorCloseCancel,
    KeyEditorCloseOk,
    LibraryDeleteBlurDown,
    LibraryDeleteBlurUp,
    LibraryDeleteInputBlurDown,
    LibraryDeleteInputBlurUp,
    LibraryLoadDirBlurDown,
    LibraryLoadDirBlurUp,
    LibraryLoadDirInputBlurDown,
    LibraryLoadDirInputBlurUp,
    LibraryPasteBlurDown,
    LibraryPasteBlurUp,
    LibraryPasteInputBlurDown,
    LibraryPasteInputBlurUp,
    LibrarySearchBlurDown,
    LibrarySearchBlurUp,
    LibrarySearchInputBlurDown,
    LibrarySearchInputBlurUp,
    LibrarySearchYoutubeBlurDown,
    LibrarySearchYoutubeBlurUp,
    LibrarySearchYoutubeInputBlurDown,
    LibrarySearchYoutubeInputBlurUp,
    LibraryTagEditorBlurDown,
    LibraryTagEditorBlurUp,
    LibraryTagEditorInputBlurDown,
    LibraryTagEditorInputBlurUp,
    LibraryYankBlurDown,
    LibraryYankBlurUp,
    LibraryYankInputBlurDown,
    LibraryYankInputBlurUp,
    PlaylistDeleteBlurDown,
    PlaylistDeleteBlurUp,
    PlaylistDeleteInputBlurDown,
    PlaylistDeleteInputBlurUp,
    PlaylistDeleteAllBlurDown,
    PlaylistDeleteAllBlurUp,
    PlaylistDeleteAllInputBlurDown,
    PlaylistDeleteAllInputBlurUp,
    PlaylistShuffleBlurDown,
    PlaylistShuffleBlurUp,
    PlaylistShuffleInputBlurDown,
    PlaylistShuffleInputBlurUp,
    PlaylistModeCycleBlurDown,
    PlaylistModeCycleBlurUp,
    PlaylistModeCycleInputBlurDown,
    PlaylistModeCycleInputBlurUp,
    PlaylistPlaySelectedBlurDown,
    PlaylistPlaySelectedBlurUp,
    PlaylistPlaySelectedInputBlurDown,
    PlaylistPlaySelectedInputBlurUp,
    PlaylistAddFrontBlurDown,
    PlaylistAddFrontBlurUp,
    PlaylistAddFrontInputBlurDown,
    PlaylistAddFrontInputBlurUp,
    PlaylistSearchBlurDown,
    PlaylistSearchBlurUp,
    PlaylistSearchInputBlurDown,
    PlaylistSearchInputBlurUp,
    PlaylistSwapDownBlurDown,
    PlaylistSwapDownBlurUp,
    PlaylistSwapDownInputBlurDown,
    PlaylistSwapDownInputBlurUp,
    PlaylistSwapUpBlurDown,
    PlaylistSwapUpBlurUp,
    PlaylistSwapUpInputBlurDown,
    PlaylistSwapUpInputBlurUp,
    RadioOkBlurUp,
    RadioOkBlurDown,
}
// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ColorEditor(IdColorEditor),
    DBListCriteria,
    DBListSearchResult,
    DBListSearchTracks,
    DeleteConfirmRadioPopup,
    DeleteConfirmInputPopup,
    ErrorPopup,
    GeneralSearchInput,
    GeneralSearchTable,
    GlobalListener,
    HelpPopup,
    KeyEditor(IdKeyEditor),
    Label,
    Library,
    Lyric,
    MessagePopup,
    Playlist,
    Progress,
    QuitPopup,
    TagEditor(IdTagEditor),
    YoutubeSearchInputPopup,
    YoutubeSearchTablePopup,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdTagEditor {
    CounterDelete,
    HelpPopup,
    LabelHint,
    InputArtist,
    InputTitle,
    RadioTag,
    SelectLyric,
    TableLyricOptions,
    TextareaLyric,
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdColorEditor {
    RadioOk,
    LabelHint,
    ThemeSelect,
    HelpPopup,
    LibraryLabel,
    LibraryForeground,
    LibraryBackground,
    LibraryBorder,
    LibraryHighlight,
    LibraryHighlightSymbol,
    PlaylistLabel,
    PlaylistForeground,
    PlaylistBackground,
    PlaylistBorder,
    PlaylistHighlight,
    PlaylistHighlightSymbol,
    ProgressLabel,
    ProgressForeground,
    ProgressBackground,
    ProgressBorder,
    LyricLabel,
    LyricForeground,
    LyricBackground,
    LyricBorder,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdKeyEditor {
    DatabaseAddAll,
    DatabaseAddAllInput,
    GlobalColorEditor,
    GlobalColorEditorInput,
    GlobalDown,
    GlobalDownInput,
    GlobalGotoBottom,
    GlobalGotoBottomInput,
    GlobalGotoTop,
    GlobalGotoTopInput,
    GlobalHelp,
    GlobalHelpInput,
    GlobalKeyEditor,
    GlobalKeyEditorInput,
    GlobalLayoutTreeview,
    GlobalLayoutTreeviewInput,
    GlobalLayoutDatabase,
    GlobalLayoutDatabaseInput,
    GlobalLeft,
    GlobalLeftInput,
    GlobalLyricAdjustForward,
    GlobalLyricAdjustForwardInput,
    GlobalLyricAdjustBackward,
    GlobalLyricAdjustBackwardInput,
    GlobalLyricCycle,
    GlobalLyricCycleInput,
    GlobalPlayerToggleGapless,
    GlobalPlayerToggleGaplessInput,
    GlobalPlayerTogglePause,
    GlobalPlayerTogglePauseInput,
    GlobalPlayerNext,
    GlobalPlayerNextInput,
    GlobalPlayerPrevious,
    GlobalPlayerPreviousInput,
    GlobalPlayerSeekForward,
    GlobalPlayerSeekForwardInput,
    GlobalPlayerSeekBackward,
    GlobalPlayerSeekBackwardInput,
    GlobalPlayerSpeedUp,
    GlobalPlayerSpeedUpInput,
    GlobalPlayerSpeedDown,
    GlobalPlayerSpeedDownInput,
    GlobalQuit,
    GlobalQuitInput,
    GlobalRight,
    GlobalRightInput,
    GlobalUp,
    GlobalUpInput,
    GlobalVolumeDown,
    GlobalVolumeDownInput,
    GlobalVolumeUp,
    GlobalVolumeUpInput,
    HelpPopup,
    LabelHint,
    LibraryDelete,
    LibraryDeleteInput,
    LibraryLoadDir,
    LibraryLoadDirInput,
    LibraryPaste,
    LibraryPasteInput,
    LibrarySearch,
    LibrarySearchInput,
    LibrarySearchYoutube,
    LibrarySearchYoutubeInput,
    LibraryTagEditor,
    LibraryTagEditorInput,
    LibraryYank,
    LibraryYankInput,
    PlaylistDelete,
    PlaylistDeleteAll,
    PlaylistShuffle,
    PlaylistModeCycle,
    PlaylistPlaySelected,
    PlaylistAddFront,
    PlaylistSearch,
    PlaylistDeleteInput,
    PlaylistDeleteAllInput,
    PlaylistShuffleInput,
    PlaylistModeCycleInput,
    PlaylistPlaySelectedInput,
    PlaylistAddFrontInput,
    PlaylistSearchInput,
    PlaylistSwapDown,
    PlaylistSwapDownInput,
    PlaylistSwapUp,
    PlaylistSwapUpInput,
    RadioOk,
}

// StatusLine shows the status of download
#[derive(Copy, Clone)]
pub enum StatusLine {
    Default,
    Success,
    Running,
    Error,
}

pub enum SearchLyricState {
    Finish(Vec<SongTag>),
}

pub struct UI {
    model: Model,
}

impl UI {
    /// Instantiates a new Ui
    pub fn new(config: &Settings) -> Self {
        let mut model = Model::new(config);
        model.init_config();
        Self { model }
    }
    /// ### run
    ///
    /// Main loop for Ui thread
    pub fn run(&mut self) {
        self.model.init_terminal();
        // self.model.player.playlist.playlist_load().ok();
        // Main loop
        let mut progress_interval = 0;
        while !self.model.quit {
            #[cfg(feature = "mpris")]
            self.model.update_mpris();

            self.model.te_update_lyric_options();
            // self.model.update_playlist_items();
            self.model.update_components();
            self.model.update_lyric();
            // #[cfg(not(any(feature = "mpv", feature = "gst")))]
            // self.model.progress_update();
            self.model.update_player_msg();

            if progress_interval == 0 {
                self.model.run();

                #[cfg(not(any(feature = "mpv", feature = "gst")))]
                self.model.player.get_progress().ok();
            }
            progress_interval += 1;
            if progress_interval >= 80 {
                progress_interval = 0;
            }

            match self.model.app.tick(PollStrategy::Once) {
                Err(err) => {
                    self.model
                        .mount_error_popup(format!("Application error: {}", err).as_str());
                }
                Ok(messages) if !messages.is_empty() => {
                    // NOTE: redraw if at least one msg has been processed
                    self.model.redraw = true;
                    for msg in messages {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = self.model.update(msg);
                        }
                    }
                }
                _ => {}
            }
            // Check whether to force redraw
            self.check_force_redraw();
            self.model.view();
            // sleep(Duration::from_millis(20));
        }
        assert!(self.model.player.playlist.save().is_ok());
        if let Err(e) = self.model.config.save() {
            eprintln!("{}", e);
        };
        // assert!(self.model.clear_photo().is_ok());

        self.model.finalize_terminal();
    }

    fn check_force_redraw(&mut self) {
        // If source are loading and at least 100ms has elapsed since last redraw...
        // if self.model.status == Status::Running {
        if self.model.since_last_redraw() >= FORCED_REDRAW_INTERVAL {
            self.model.force_redraw();
        }
        // }
    }
}
