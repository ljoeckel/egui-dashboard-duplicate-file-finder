use std::rc::Rc;
use std::sync::MutexGuard;
use eframe::egui::{self, *};
use eframe::egui::scroll_area::ScrollBarVisibility;
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};
use egui_modal::*;


pub fn mediatable(
    ui: &mut egui::Ui,
    active_theme: &Rc<dyn Aesthetix>,
    duplicates: &MutexGuard<Vec<String>>,
    mut checked: MutexGuard<Vec<bool>>)
{
    let modal = Modal::new(ui.ctx(), "confirm_dialog");

    let available_height = ui.available_height();
    let row_height = egui::TextStyle::Body
        .resolve(ui.style())
        .size
        .max(ui.spacing().interact_size.y);
    let chars_per_line = (ui.available_width() / 9.3) as usize; //FIXME: Find a better solution to calculate exact chars

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
            let mut selpath: Vec<String> = Vec::new();
            for i in 0..checked.len() {
                if checked[i] {
                    selpath.push(duplicates[i].to_string());
                }
            }

            header.col(|ui| {
                // Create Modal Dialog for deletion
                modal.show(|ui| {
                    modal.title(ui, "Delete selected files?");
                    modal.frame(ui, |ui| {
                        if selpath.len() == 1 {
                            modal.body(ui, "Delete the one selected?");
                        } else {
                            modal.body(ui, format!("Delete the {} selected files?", selpath.len()));
                        }
                    });
                    modal.buttons(ui, |ui| {
                        // After clicking, the modal is automatically closed
                        if modal.button(ui, "DELETE").clicked() {
                            println!("DELETE");
                        };
                        if modal.button(ui, "CANCEL").clicked() {
                            println!("Canceled");
                        }
                    });
                });

                // Add Delete Button in the header
                if ui.add_enabled(checked.iter().any(|&e| e == true), egui::Button::new("\u{e613} Delete")).clicked() {
                    for i in 0..checked.len() {
                        if checked[i] {
                            println!("Delete {}", &duplicates[i]);
                        }
                    }
                    modal.open();
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