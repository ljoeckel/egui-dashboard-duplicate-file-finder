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
            sa.add_space(10.0);

            // Zoom-Factor
            sa.heading("Zoom factor");
            let slider = egui::Slider::new(&mut state.zoom_factor, 0.4..=2.0).step_by(0.10);
            if sa.add(slider).drag_stopped() {
                ctx.set_zoom_factor(state.zoom_factor);
            }
            sa.add_space(10.0);

            sa.heading("egui Settings");
            sa.checkbox(&mut state.settings_window_open, "\u{1F527} egui-Settings");
            egui::Window::new("\u{1F527} egui-Settings")
                .open(&mut state.settings_window_open)
                .vscroll(true)
                .show(&ctx, |ui| {
                    ctx.settings_ui(ui);
                });
        });
}
