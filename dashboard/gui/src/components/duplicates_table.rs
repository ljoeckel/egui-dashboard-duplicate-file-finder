use std::rc::Rc;
use std::sync::MutexGuard;
use eframe::egui::{self, *};
use eframe::egui::scroll_area::ScrollBarVisibility;
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};


pub fn mediatable(
    ui: &mut egui::Ui,
    active_theme: &Rc<dyn Aesthetix>,
    duplicates: &MutexGuard<Vec<String>>,
    mut checked: MutexGuard<Vec<bool>>)
{
    let available_height = ui.available_height();
    let row_height = egui::TextStyle::Body
        .resolve(ui.style())
        .size
        .max(ui.spacing().interact_size.y);
    let chars_per_line = (ui.available_width() / 9.3) as usize; //FIXME: Find a better solution to calculate exact chars

    // egui::Align::Center
    // bottom_panel_widget.show(context, |ui_panel| {
    // ui_panel.with_layout(egui::Layout::right_to_left(alignment), |ui_panel_layout| {

    TableBuilder::new(ui)
        .striped(true)
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
        .vscroll(true)
        .stick_to_bottom(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::remainder())
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height)
        .sense(egui::Sense::click())
        .header(row_height * 2.0, |mut header| {
            header.col(|ui| {
                if ui.add_enabled(checked.iter().any(|&e| e == true), egui::Button::new("\u{e613} Delete")).clicked() {
                    for i in 0..checked.len() {
                        if checked[i] {
                            println!("Delete {}", &duplicates[i]);
                        }
                    }
                }
            });
            header.col(|ui| {
                ui.strong("Path");
            });
        })
        .body(|mut body| {
            for i in 0..duplicates.len() {
                body.row(row_height, |mut row| {
                    let row_index = row.index();
                    row.col(|ui| {
                        ui.checkbox(&mut checked[i], "");
                    });
                    row.col(|ui| {
                        // Show the nn right characters in the table
                        let s = &duplicates[i];
                        let len = utf8_slice::len(s);
                        let utf: &str;
                        if len >= chars_per_line {
                            utf = utf8_slice::from(s, len - chars_per_line);
                        } else {
                            utf = utf8_slice::from(s, 0);
                        }

                        // Change text color when selected
                        let fg_color: Color32;
                        if checked[i] {
                            fg_color = active_theme.fg_error_text_color_visuals();
                        } else {
                            fg_color = active_theme.fg_primary_text_color_visuals().unwrap();
                        }

                        // Create a tooltip when line longer than visible part
                        let resp = ui.add(Label::new(RichText::new(utf.to_string()).color(fg_color)));
                        if len >= chars_per_line {
                            resp.on_hover_text(s);
                        }
                    });

                    // Select/Deselect line or use checkbox
                    if row.response().clicked() {
                        checked[row_index] = !checked[row_index];
                    }
                }); // row
            } // for
        }); // body
}