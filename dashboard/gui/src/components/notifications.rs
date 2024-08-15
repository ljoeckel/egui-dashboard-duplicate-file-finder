//! Notifications bar at the bottom of the app

use crate::app::ApplicationState;
use eframe::egui;

// Text to display when there are no notifications
const NO_NOTIFICATIONS: &str = "No new notifications";

#[derive(Debug)]
enum MessageType {
    INFO,
    WARNING,
    ERROR,
}

/// Renders the notifications bar at the bottom of the app
#[derive(Debug)]
pub struct NotificationBar {
    /// Message to display in the notifications bar
    message: String,
    message_type: MessageType,

    /// Expand the bottom bar
    expanded: bool,
    // Calculated progress for the progressbar. Call set_progress first.
    progress: f32,
}

impl NotificationBar {
    /// Renders the notifications bar at the bottom of the app
    pub fn new() -> Self {
        Self {
            message: NO_NOTIFICATIONS.to_owned(),
            message_type: MessageType::INFO,
            expanded: false,
            progress: 0.0,
        }
    }

    pub fn info(&mut self, message: &str) {
        self.set_message(message, MessageType::INFO);
    }

    pub fn warn(&mut self, message: &str) {
        self.set_message(message,MessageType::WARNING);
    }

    pub fn error(&mut self, message: &str) {
        self.set_message(message, MessageType::ERROR);
    }

    pub fn set_progress(&mut self, progress: f32, info: &str) {
        self.progress = progress;
        if !info.is_empty() {
            self.set_message(info, MessageType::INFO);
        }
    }

    pub fn clear(&mut self) {
        self.progress = 0.0;
        self.set_message("", MessageType::INFO);
    }

    fn set_message(&mut self, message: &str, message_type: MessageType) {
        self.message = message.to_string();
        self.message_type = message_type;
    }

    /// Renders the bottom bar
    pub fn ui(&mut self, context: &egui::Context, state: &ApplicationState) {
        let mut bottom_panel_widget = egui::TopBottomPanel::bottom("bottom_panel").frame(
            egui::Frame::default()
                .fill(state.active_theme.bg_secondary_color_visuals())
                .inner_margin(egui::vec2(10.0, 5.0)),
        );

        let alignment = if !self.expanded {
            bottom_panel_widget = bottom_panel_widget.max_height(26.0);
            egui::Align::TOP
        } else {
            egui::Align::Center
        };

        bottom_panel_widget.show(context, |ui_panel| {
            ui_panel.with_layout(egui::Layout::right_to_left(alignment), |ui_panel_layout| {
                if !self.expanded {
                    if ui_panel_layout.small_button("Expand").clicked() {
                        self.expanded = true;
                    }
                } else if ui_panel_layout.button("Reduce").clicked() {
                    self.expanded = false;
                }

                if self.progress > 0.0 && self.progress < 1.0 {
                    let progress_bar = egui::ProgressBar::new(self.progress)
                        .show_percentage()
                        .desired_width(400.0);
                    ui_panel_layout.add(progress_bar);
                }

                ui_panel_layout.with_layout(
                    egui::Layout::left_to_right(alignment),
                    |ui_panel_layout_label| {
                        let (bg, fg);
                        match self.message_type {
                            MessageType::INFO => (bg = state.active_theme.bg_secondary_color_visuals(), fg = state.active_theme.fg_success_text_color_visuals()),
                            MessageType::WARNING => (bg = state.active_theme.bg_triage_color_visuals(), fg = state.active_theme.fg_warn_text_color_visuals()),
                            MessageType::ERROR => (bg = state.active_theme.bg_contrast_color_visuals(), fg = state.active_theme.fg_error_text_color_visuals()),
                        };
                        ui_panel_layout_label
                            .add(egui::Label::new(egui::RichText::new(&self.message)
                                .background_color(bg)
                                .color(fg)
                            ).wrap())
                    },
                );
            });
        });
    }
}
