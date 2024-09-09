use std::rc::Rc;
use std::sync::MutexGuard;
use eframe::egui::{self, *};
use eframe::egui::scroll_area::ScrollBarVisibility;
use egui_aesthetix::Aesthetix;
use egui_extras::{Column, TableBuilder};
use egui_modal::*;
use std::path::Path;
use std::collections::HashMap;

const CHARS_PER_LINE: [(f32, f32, f32); 9] = [
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
    for (zf, width, cpl) in CHARS_PER_LINE.iter() {
        if zf.eq(&zoom_factor) {
            return ((available_width + 17.0 * zoom_factor) / (width / cpl)) as usize;
        }
    }
    82
}

// Return a list of index positions where the checkox from checked is selected
fn get_checked_idxs(checked: &mut MutexGuard<Vec<bool>>) -> Vec<usize> {
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

    checked_idxs
}

fn delete_checked_duplicates(duplicates: &mut MutexGuard<Vec<HashMap<String, String>>>, checked: &mut MutexGuard<Vec<bool>>) {
    // Get the index-positions
    let checked_idxs = get_checked_idxs(checked);

    // Delete the files
    for idx in checked_idxs.into_iter().rev() {
        let map = &duplicates[idx];
        let path = map.get("PATH").unwrap();

        // Delete the file with the given path. On success return true
        println!("TODO! Delete file {}", path);

        // Remove from duplicates/checked
        duplicates.remove(idx);
        //&duplicates.remove(idx);
        checked.remove(idx);
    }
}

pub fn mediatable(
    ui: &mut egui::Ui,
    duplicates: &mut MutexGuard<Vec<HashMap<String, String>>>,
    checked: &mut MutexGuard<Vec<bool>>,
    active_theme: &Rc<dyn Aesthetix>,
    zoom_factor: f32,
)
{
    // Calculate Sizes
    let available_width = ui.available_width();
    let available_height = ui.available_height();
    let row_height = egui::TextStyle::Body
        .resolve(ui.style())
        .size
        .max(ui.spacing().interact_size.y);

    // Create Modal Dialog
    let modal_style: ModalStyle;
    if ui.visuals().dark_mode {
        modal_style = ModalStyle {
            overlay_color: Color32::from_rgba_premultiplied(0x20, 0x20, 0x10, 80),
            ..ModalStyle::default()
        };
    } else {
        modal_style = ModalStyle {
            overlay_color: Color32::from_rgba_premultiplied(0x0f, 0x0e, 0x08, 80),
            ..ModalStyle::default()
        };
    }
    let modal = Modal::new(ui.ctx(), "confirm_dialog").with_style(&modal_style);

    // Create the Table
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
                // Create Modal Dialog for deletion if something checked
                if cnt_checked > 0 {
                    modal.show(|ui| {
                        modal.title(ui, "Delete selected files?");
                        modal.frame(ui, |ui| {
                            modal.body(ui, format!("Delete the {} selected file(s)?", cnt_checked));
                        });
                        modal.buttons(ui, |ui| {
                            if modal.button(ui, "DELETE").clicked() {
                                delete_checked_duplicates(duplicates, checked);
                            };
                            if modal.button(ui, "CANCEL").clicked() {
                                // Do nothing
                            };

                        });
                    });
                }

                // Add Delete Button in the header if something checked
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
                    let map = &duplicates[row_index];
                    let s = map.get("PATH").unwrap();
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

                // Contextmenu on Row
                row.response().on_hover_ui(|ui| {
                    let map = &duplicates[row_index];

                    // Show sticky path in the first line
                    let path = Path::new(map.get("PATH").unwrap());

                    ui.label(RichText::new(path.to_str().unwrap().to_string())
                        .color(Color32::DARK_GRAY)
                        .background_color(Color32::from_rgba_premultiplied(0, 8, 32, 16))
                        .font(FontId::proportional(15.0)));
                    ui.separator();

                    ui.label(RichText::new(map.get("AlbumArtist").unwrap_or(&"".to_string()))
                        .color(Color32::DARK_BLUE)
                        .background_color(Color32::from_rgba_premultiplied(0, 8, 32, 16)));
                    ui.label(RichText::new(map.get("AlbumTitle").unwrap_or(&"".to_string()))
                        .text_style(TextStyle::Small)
                        .color(Color32::DARK_BLUE)
                        .background_color(Color32::from_rgba_premultiplied(0, 8, 32, 16)));
                    ui.label(RichText::new(map.get("TrackTitle").unwrap_or(&"".to_string()))
                        .color(Color32::DARK_BLUE)
                        .background_color(Color32::from_rgba_premultiplied(0, 8, 32, 16)));

                    egui::ScrollArea::vertical()
                        .enable_scrolling(true)
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                        .enable_scrolling(true)
                        .drag_to_scroll(true)
                        .show(ui, |ui| {
                            egui::Grid::new("duplicates_table_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    let mut vkeys = map.keys().cloned().collect::<Vec<_>>();
                                    vkeys.sort();
                                    for key in vkeys.iter() {
                                        ui.label(key);
                                        ui.label(map.get(key).unwrap());
                                        ui.end_row();
                                    }
                                }); // Grid show
                        }); // scroll Aerea
                }); // on_hoover_ui

                // Select/Deselect line or use checkbox
                if row.response().clicked() {
                    checked[row_index] = !checked[row_index];
                }
            }); // row
        }); // body
}
