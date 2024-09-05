use std::rc::Rc;
use std::sync::MutexGuard;
use eframe::egui::{self, *};
use eframe::egui::scroll_area::ScrollBarVisibility;
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};
use egui_modal::*;

const CHARS_PER_LINE: [(f32, f32, f32);9] = [
    (0.7, 1216.0, 130.0),
    (0.8, 1038.0, 116.0),
    (0.9, 899.0, 98.0),
    (1.0, 788.0, 81.0),
    (1.1, 697.0, 73.0),
    (1.2, 621.0, 61.0),
    (1.3, 557.0, 54.0),
    (1.4, 513.0, 47.0),
    (1.5, 508.0, 42.0),
];
fn chars_per_line(zoom_factor: f32, available_width: f32) -> usize {
    for (zf, width, cpl) in CHARS_PER_LINE.iter(){
        if zf.eq(&zoom_factor) {
            return ((available_width + 17.0*zoom_factor) / (width / cpl)) as usize;
        }
    }
    82
}

pub fn mediatable(
    ui: &mut egui::Ui,
    active_theme: &Rc<dyn Aesthetix>,
    duplicates: &mut MutexGuard<Vec<String>>,
    checked: &mut MutexGuard<Vec<bool>>,
    zoom_factor: f32,
)
{
    // Create Modal Dialog
    let modal = Modal::new(ui.ctx(), "confirm_dialog");

    let available_height = ui.available_height();
    let row_height = egui::TextStyle::Body
        .resolve(ui.style())
        .size
        .max(ui.spacing().interact_size.y);

    let available_width = ui.available_width();

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
            // Something to delete?
            let cnt_checked = checked.iter().filter(|&&x| x).count();

            header.col(|ui| {
                // Create Modal Dialog for deletion if something to delete
                if cnt_checked > 0 {
                    modal.show(|ui| {
                        modal.title(ui, "Delete selected files?");
                        modal.frame(ui, |ui| {
                            if cnt_checked > 1 {
                                modal.body(ui, format!("Delete the {} selected files?", cnt_checked));
                            } else {
                                modal.body(ui, "Delete the one selected?");
                            }
                        });
                        modal.buttons(ui, |ui| {
                            if modal.button(ui, "DELETE").clicked() {
                                let checked_idxs: Vec<usize> = checked.iter()
                                    .enumerate()
                                    .filter_map(|(index, &is_true)| {
                                        if is_true {
                                            Some(index)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();

                                // Delete the files
                                for idx in checked_idxs.into_iter().rev() {
                                    let path = &duplicates[idx];
                                    if delete_file(path) {
                                        // Remove from duplicates/checked
                                        let _ = &duplicates.remove(idx);
                                        checked.remove(idx);
                                    }
                                }
                            };
                            if modal.button(ui, "CANCEL").clicked() {
                                println!("Canceled");
                            }
                        });
                    });
                }

                // Add Delete Button in the header
                if ui.add_enabled(cnt_checked > 0, egui::Button::new("\u{e613} Delete")).clicked() {
                    modal.open();
                }
            });
            header.col(|ui| {
                ui.strong("Path");
            });
        })
        .body(|body| {
            body.rows(row_height, duplicates.len(), |mut row| {
                let row_index = row.index();
                row.col(|ui| {
                    ui.checkbox(&mut checked[row_index], "");
                });
                row.col(|ui| {
                    let chars_per_line = chars_per_line(zoom_factor, available_width);
                    // Show the nn right characters in the table
                    let s = &duplicates[row_index];
                    let len = utf8_slice::len(s);
                    let utf: &str;
                    if len >= chars_per_line {
                        utf = utf8_slice::from(s, len - chars_per_line);
                    } else {
                        utf = utf8_slice::from(s, 0);
                    }

                    // Change text color when selected
                    let fg_color: Color32;
                    if checked[row_index] {
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
        }); // body

    // Delete the file with the given path. On success return true
    fn delete_file(path: &String) -> bool {
        println!("Delete file {}", path);
        return true;
    }
}
