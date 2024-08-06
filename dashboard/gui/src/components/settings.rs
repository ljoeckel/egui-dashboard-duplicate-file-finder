//! Settings Tab

use crate::app::ApplicationState;
use eframe::egui;
use egui_aesthetix::Aesthetix;
use std::rc::Rc;

/// Renders the settings page
pub fn settings_ui(
    ui_root: &mut egui::Ui,
    state: &mut ApplicationState,
    themes: &[Rc<dyn Aesthetix>],
) {
    let ctx = ui_root.ctx().clone();

    egui::ScrollArea::new([false, true])
        .id_source("settings_tab_scroll_area")
        .show(ui_root, |sa| {
            sa.heading("Style");
            egui::ComboBox::from_label("Theme")
                .width(200.0)
                .selected_text(state.active_theme.name())
                .show_ui(sa, |ui_combobox| {
                    for theme in themes {
                        let res: egui::Response = ui_combobox.selectable_value(
                            &mut state.active_theme,
                            Rc::clone(theme),
                            theme.name(),
                        );
                        if res.changed() {
                            ui_combobox
                                .ctx()
                                .set_style(state.active_theme.custom_style());
                        }
                    }
                });
        });
}
