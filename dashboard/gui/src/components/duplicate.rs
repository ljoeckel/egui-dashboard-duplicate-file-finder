use crate::scanner::mediatype::{MediaGroup, ScanType};
use crate::scanner::messenger::Messenger;
use crate::scanner::scanner::scan;

use std::{
    path::Path,
    sync::MutexGuard,
    thread::{self},
    time::Duration,
    vec::Vec,
};
use eframe::egui::{Align, Button, Layout, Color32, Label, RichText, ScrollArea, Stroke, TextEdit, TextStyle};
use eframe::epaint::Vec2;

use eframe::egui;
use egui::scroll_area::ScrollBarVisibility;
use egui::{epaint::text::TextWrapMode, Ui};
use egui_file_dialog::{FileDialog};

use crate::components::notifications::NotificationBar;
use crate::components::{duplicates_table};
use egui_comps::tabbar::TabBar;
use crate::app::ApplicationState;

const BUTTON_HEIGHT: f32 = 32.0;

const TAB_COLORS: [&[Color32]; 3] = [
    &[Color32::DARK_BLUE, Color32::LIGHT_BLUE],
    &[Color32::DARK_RED, Color32::LIGHT_RED],
    &[Color32::DARK_GREEN, Color32::LIGHT_GREEN],
];

#[derive(PartialEq, Copy, Clone)]
enum ShowTab {
    Scanned,
    Errors,
    Duplicates,
}
impl ShowTab {
    pub fn from(tab_idx: usize) -> Self {
        match tab_idx {
            0 => ShowTab::Scanned,
            1 => ShowTab::Errors,
            _ => ShowTab::Duplicates,
        }
    }

    #[allow(dead_code)]
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub struct DuplicateScannerUI {
    scan_type: ScanType,
    selected_tab: usize,
    path: String,
    file_dialog: FileDialog,
    messenger: Messenger,
    scanning: bool,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl DuplicateScannerUI {
    pub fn new() -> Self {
        Self {
            scan_type: ScanType::METADATA,
            selected_tab: 0, // select first tab as default
            path: String::new(),
            file_dialog: FileDialog::new(),
            messenger: Messenger::new(),
            scanning: false,
            handle: None,
        }
    }

    fn clear(&mut self) {
        self.messenger.clear();
        self.scanning = false;
    }

    fn is_scanning(&self) -> bool {
        match self.handle.as_ref() {
            Some(handle) => return !handle.is_finished(),
            None => false,
        }
    }

    fn have_results(&self) -> bool {
        self.messenger.cntres() > 0 || self.messenger.cnterr() > 0 || self.messenger.cntstd() > 0
    }

    fn get_checked(&self) -> MutexGuard<Vec<bool>> {
        self.messenger.checked()
    }

    fn get_tab_color(&self, ui: &Ui) -> Color32 {
        let mut dark_idx = 0;
        if ui.visuals().dark_mode {
            dark_idx = 1;
        }
        TAB_COLORS[self.selected_tab][dark_idx]
    }
}


/// Renders the duplicate fild finder page
pub fn duplicate_ui(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    state: &mut ApplicationState,
    dss: &mut DuplicateScannerUI,
    media_groups: Vec<MediaGroup>,
    notification_bar: &mut NotificationBar,
) {
    let is_scanning = dss.is_scanning();
    let have_results = dss.have_results();

    // Update the NotificationBar
    notification_bar.info(&dss.messenger.info());
    notification_bar.set_progress(dss.messenger.progress(), "");

    ui.add_space(10.0);

    egui::Grid::new("my_grid")
        .num_columns(2)
        .spacing([30.0, 4.0])
        .striped(false)
        .show(ui, |ui| {
            ui.strong("Path:");
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut dss.path)
                    .desired_width(ui.available_width() - 50.0));

                // Open Directory Symbol
                if ui.add_enabled(!is_scanning, egui::Button::new(" \u{e613} "))
                    .clicked() {
                    dss.file_dialog.select_directory();
                }
            });
            ui.end_row();

            // ScanType
            ui.strong("ScanType:");
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", dss.scan_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut dss.scan_type, ScanType::BINARY, "Binary");
                        ui.selectable_value(&mut dss.scan_type, ScanType::METADATA, "Metadata");
                    });

                // Scan / Abort Buttons
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    // Abort
                    if ui.add_enabled(
                        is_scanning,
                        Button::new("ABORT")
                            .min_size(Vec2::new(80.0, BUTTON_HEIGHT))
                            .stroke(Stroke::new(1.0, Color32::LIGHT_RED)))
                        .clicked() {
                        dss.messenger.stop();
                    }

                    ui.add_space(5.0);

                    // Break
                    if ui.add_enabled(
                        is_scanning,
                        Button::new("Interrupt")
                            .min_size(Vec2::new(80.0, BUTTON_HEIGHT))
                            .stroke(Stroke::new(1.0, Color32::LIGHT_RED)))
                        .clicked() {
                        dss.messenger.interrupt();
                    }

                    ui.add_space(5.0);

                    // Start Scan
                    if ui.add_enabled(
                        !is_scanning && !dss.path.is_empty(),
                        Button::new("SCAN")
                            .min_size(Vec2::new(80.0, BUTTON_HEIGHT))
                            .stroke(Stroke::new(1.0, Color32::LIGHT_GREEN)))
                        .clicked() {
                        dss.clear();
                        notification_bar.clear();

                        let messenger = dss.messenger.clone();
                        let path = dss.path.clone();
                        let scan_type = dss.scan_type;
                        dss.handle = Some(thread::spawn(move || {
                            scan(Path::new(&path), scan_type, media_groups, messenger);
                        }));
                    } // clicked
                }) // with_layout;
            }); // ui.horizontal
            ui.end_row();
        });

    ui.add_space(15.0);

    // Add the TabBar
    let cols: Vec<String> = vec! {
        format!("Scanned [{}]", dss.messenger.cntstd()),
        format!("Problems [{}]", dss.messenger.cnterr()),
        format!("Duplicates [{}]", dss.messenger.cntres()),
    };

    ui.add_enabled(have_results, TabBar::new(cols, &mut dss.selected_tab, &ui.visuals())
        .selected_bg(Color32::from_rgb(0xf6, 0xb1, 0x7a), Color32::from_rgb(0x6e, 0x85, 0xb7))
        .selected_fg(Color32::BLACK, Color32::WHITE)
        .hover_bg(Color32::from_rgb(0x70, 0x77, 0xa1), Color32::from_rgb(218, 207, 181))
        .hover_fg(Color32::WHITE, Color32::BLACK)
        .bg(Color32::from_rgb(0x42, 0x47, 0x69), Color32::from_rgb(226, 221, 213))
        .fg(Color32::LIGHT_GRAY, Color32::DARK_GRAY),
    );

    // Open FileDialog
    match dss.file_dialog.update(ctx).selected() {
        Some(path) => {
            dss.path.clear();
            dss.path.push_str(path.to_str().unwrap());
        }
        _ => ()
    }

    // Scroll-Area LOG
    let row_height = ui.text_style_height(&TextStyle::Monospace);
    let scroll_area = ScrollArea::vertical()
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
        .auto_shrink(false)
        .hscroll(true)
        .stick_to_bottom(true);

    if ShowTab::from(dss.selected_tab) == ShowTab::Duplicates {
        let mut stack = dss.messenger.reslog();
        let mut checked = dss.get_checked();
        duplicates_table::mediatable(ui, state, &mut stack, &mut checked);
    } else {
        let color = dss.get_tab_color(&ui);

        if ShowTab::from(dss.selected_tab) == ShowTab::Scanned {
            let stack = dss.messenger.stdlog(); // TODO:
            scroll_area.show_rows(ui, row_height, stack.len(), |ui, row_range| {
                for row in row_range {
                    let msg = stack.get(row).unwrap();
                    let rt = RichText::new(msg).color(color);
                    ui.add(Label::new(rt).wrap_mode(TextWrapMode::Extend));
                }
            });
        } else if ShowTab::from(dss.selected_tab) == ShowTab::Errors {
            let stack = dss.messenger.errlog(); //TODO:
            scroll_area.show_rows(ui, row_height, stack.len(), |ui, row_range| {
                for row in row_range {
                    let msg = stack.get(row).unwrap();
                    let rt = RichText::new(msg).color(color);
                    ui.add(Label::new(rt).wrap_mode(TextWrapMode::Extend));
                }
            });
        }
        if is_scanning && !ctx.has_requested_repaint() {
            ctx.request_repaint();
        }
    }

    if !ctx.has_requested_repaint() {
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
