//! Debug Tab

use crate::app::ApplicationState;
use eframe::egui;

/// Renders the debug page
pub fn debug_ui(ui_root: &mut egui::Ui, state: &mut ApplicationState) {
    let ctx = ui_root.ctx().clone();

    egui::ScrollArea::new([false, true])
        .id_source("settings_tab_scroll_area")
        .show(ui_root, |sa| {
            sa.horizontal(|ui| {
                ui.checkbox(&mut state.inspector_window_open, "\u{1F50D} Inspection");
                ui.checkbox(&mut state.memory_window_open, "\u{1F4DD} Memory");
            });
            sa.add_space(20.0);

            egui::Window::new("\u{1F50D} Inspection")
                .open(&mut state.inspector_window_open)
                .vscroll(true)
                .show(&ctx, |ui| {
                    ctx.inspection_ui(ui);
                });

            egui::Window::new("\u{1F4DD} Memory")
                .open(&mut state.memory_window_open)
                .resizable(false)
                .show(&ctx, |ui| {
                    ctx.memory_ui(ui);
                });
        });
}
