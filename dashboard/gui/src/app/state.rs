//! holds the state of the application

use egui_aesthetix::Aesthetix;
use std::rc::Rc;

/// The different tabs of the application
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tab {
    /// The home tab
    Home,
    /// The settings tab
    Settings,
    /// The logs tab
    Logs,
    /// The debug tab
    Debug,
    /// The about tab
    About,
}

/// Application state
#[derive(Debug)]
pub struct ApplicationState {
    /// The currently selected tab
    pub active_tab: Tab,
    /// The active theme
    pub active_theme: Rc<dyn Aesthetix>,

    pub settings_window_open: bool,
    pub inspector_window_open: bool,
    pub memory_window_open: bool,
    pub zoom_factor: f32,

    // Filter in duplicates_table.rs
    pub filter_unknown: bool,
    pub filter_musicbrainz: bool,
}

impl ApplicationState {
    /// Create a new state with an active theme
    #[must_use]
    pub const fn new(active_theme: Rc<dyn Aesthetix>) -> Self {
        Self {
            active_tab: Tab::Home,
            active_theme,
            settings_window_open: false,
            inspector_window_open: false,
            memory_window_open: false,
            zoom_factor: 1.0,

            filter_unknown: true,
            filter_musicbrainz: true,

        }
    }
}
