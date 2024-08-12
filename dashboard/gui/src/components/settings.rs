//! Settings Tab
use crate::{app::ApplicationState, scanner::mediatype::MediaType};
use eframe::egui::{self, util::undoer::Settings};
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};
use std::rc::Rc;

// struct TabContent {
//     media_types: Vec<MediaType>,
//     checked: bool,
// }
// impl egui_dock::TabViewer for TabContent {
//     type Tab = String;

//     fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
//         (&*tab).into()
//     }

//     fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
//         let available_height = ui.available_height();
//         let row_height = egui::TextStyle::Body
//             .resolve(ui.style())
//             .size
//             .max(ui.spacing().interact_size.y);

//         let media_types = &mut self.media_types; // Important to use boolean from struct as control for checkbox
//         let checked = &mut self.checked;

//         let mut table = TableBuilder::new(ui)
//             .striped(true)
//             .resizable(false)
//             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
//             .column(Column::exact(80.0))
//             .column(Column::auto())
//             .column(Column::remainder())
//             .min_scrolled_height(0.0)
//             .max_scroll_height(available_height);

//         table = table.sense(egui::Sense::click());

//         let table = table.header(row_height, |mut header| {
//             header.col(|ui| {
//                 ui.strong("Extension");
//             });
//             header.col(|ui| {
//                 ui.horizontal(|ui| {
//                     ui.strong("Enabled");
//                 });
//             });
//             header.col(|ui| {
//                 let cb = ui.checkbox(checked, &tab.clone());
//                 if cb.clicked() {
//                     println!("cliecked");
//                     for i in 0..media_types.len() {
//                         media_types[i].selected = !media_types[i].selected;
//                         println!("sel={}", media_types[i].selected);
//                     }
//                 }
//             });
//         });

//         table.body(|mut body| {
//             for i in 0..media_types.len() {
//                 if media_types[i].group.eq(tab) {
//                     body.row(row_height, |mut row| {
//                         row.col(|ui| {
//                             ui.label(media_types[i].extension.clone())
//                                 .on_hover_text(media_types[i].description.clone());
//                         });
//                         row.col(|ui| {
//                             ui.checkbox(&mut media_types[i].selected, "");
//                         });
//                         row.col(|ui| {
//                             ui.label("");
//                         });
//                     }); // body
//                 }
//             } // for types
//         });
//     }
// }

pub struct SettingsUI {
    pub media_types: Vec<MediaType>,
    pub media_groups: Vec<String>,
    media_groups_checked: Vec<bool>,
    //tree: DockState<String>,
}
impl SettingsUI {
    pub fn new() -> Self {
        let media_types = MediaType::load_types();
        let media_groups = MediaType::group_names(&media_types);
        let media_groups_checked = vec![true; media_groups.len()];

        //let tree = DockState::new(media_groups.clone());
        Self {
            media_types,
            media_groups,
            media_groups_checked,
            //tree,
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

        // DockArea::new(&mut self.tree)
        //     .style(Style::from_egui(ctx.style().as_ref()))
        //     .show_close_buttons(false)
        //     .draggable_tabs(false)
        //     .show_tab_name_on_hover(true)
        //     .show_window_close_buttons(false)
        //     .show_window_collapse_buttons(false)
        //     .show_inside(
        //         ui,
        //         &mut TabContent {
        //             media_types: self.media_types.clone(),
        //             checked: self.checked,
        //         },
        //     );

        // Media-Group checkboxes
        ui.horizontal(|ui| {
            let checked = &mut self.media_groups_checked;
            for (i, group) in self.media_groups.iter().enumerate() {
                if ui.checkbox(&mut checked[i], group).clicked() {
                    for j in 0..self.media_types.len() {
                        if self.media_types[j].group == String::from(group) {
                            self.media_types[j].selected = checked[i];
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
                if media_types[i].is_selected(&self.media_groups, &self.media_groups_checked) {
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

    pub fn part_ui(
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
    }
}
