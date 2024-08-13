//! Settings Tab
use crate::{
    app::ApplicationState,
    scanner::mediatype::{MediaGroup, MediaType},
};
use eframe::egui::{self};
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};
use std::rc::Rc;

pub struct SettingsUI {
    pub media_types: Vec<MediaType>,
    pub groups: Vec<MediaGroup>,
}

impl SettingsUI {
    pub fn new() -> Self {
        Self {
            media_types: MediaType::load_types(),
            groups: MediaType::load_groups(),
        }
    }

    /// Renders the settings page
    pub fn settings_ui(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut ApplicationState,
        themes: &[Rc<dyn Aesthetix>],
    ) {
        let ctx = ui.ctx().clone();

        egui::ScrollArea::new([false, true])
            .id_source("settings_tab_scroll_area")
            .show(ui, |sa| {
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

        ui.add_space(20.0);

        ui.heading("Media-Types");
        // Media Groups select/deselect
        ui.horizontal(|ui| {
            for i in 0..self.groups.len() {
                let name = self.groups[i].name.clone();
                if ui.checkbox(&mut self.groups[i].selected, &name).clicked() {
                    // Update the MediaType's of the group
                    for j in 0..self.media_types.len() {
                        if self.media_types[j].group.eq(&name) {
                            self.media_types[j].selected = self.groups[i].selected;
                        }
                    }
                }
            }
        });

        let available_height = ui.available_height();
        let row_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(80.0))
            .column(Column::exact(80.0))
            .column(Column::exact(80.0))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        table = table.sense(egui::Sense::click());

        let table = table.header(row_height * 2.0, |mut header| {
            header.col(|ui| {
                ui.strong("Extension");
            });
            header.col(|ui| {
                ui.strong("Enabled");
            });
            header.col(|ui| {
                ui.strong("Group");
            });
        });

        table.body(|mut body| {
            let media_types = &mut self.media_types; // Important to use boolean from struct as control for checkbox
            for i in 0..media_types.len() {
                if MediaType::is_group_selected(&self.groups, media_types[i].group.as_str()) {
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(media_types[i].extension.clone())
                                .on_hover_text(media_types[i].description.clone());
                        });
                        row.col(|ui| {
                            ui.checkbox(&mut media_types[i].selected, "");
                        });
                        row.col(|ui| {
                            ui.label(media_types[i].group.clone());
                        });
                    }); // body
                } // is_selected
            } // for types
        });
    }
}
