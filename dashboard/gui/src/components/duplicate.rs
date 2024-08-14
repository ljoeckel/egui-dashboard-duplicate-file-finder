use crate::scanner::mediatype::{Control, MediaGroup, ScanType};
use crate::scanner::messenger::Messenger;
use crate::scanner::scanner::scan;

use std::{
    path::Path,
    sync::MutexGuard,
    thread::{self},
    time::Duration,
};

use eframe::egui;
use eframe::egui::{Color32, Label, RichText, ScrollArea, TextStyle};
use egui::scroll_area::ScrollBarVisibility;
use egui::{epaint::text::TextWrapMode, Ui};
use egui_file_dialog::FileDialog;

#[derive(PartialEq)]
enum ShowView {
    Scanned,
    Errors,
    Duplicates,
}

pub struct DuplicateScannerUI {
    view: ShowView,
    path: String,
    file_dialog: FileDialog,
    messenger: Messenger,
    scanning: bool,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Default for DuplicateScannerUI {
    fn default() -> Self {
        Self {
            view: ShowView::Scanned,
            path: String::new(),
            file_dialog: FileDialog::new(),
            messenger: Messenger::new(),
            scanning: false,
            handle: None,
        }
    }
}

impl DuplicateScannerUI {
    pub fn new() -> Self {
        Self {
            ..Default::default()
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

    fn color(&self, ui: &Ui) -> (MutexGuard<Vec<String>>, Color32) {
        match self.view {
            ShowView::Scanned => {
                let stack = self.messenger.stdlog.lock().unwrap();
                if ui.visuals().dark_mode {
                    (stack, Color32::LIGHT_BLUE)
                } else {
                    (stack, Color32::DARK_BLUE)
                }
            }
            ShowView::Duplicates => {
                let stack = self.messenger.reslog.lock().unwrap();
                if ui.visuals().dark_mode {
                    (stack, Color32::LIGHT_GREEN)
                } else {
                    (stack, Color32::DARK_GREEN)
                }
            }
            ShowView::Errors => {
                let stack = self.messenger.errlog.lock().unwrap();
                if ui.visuals().dark_mode {
                    (stack, Color32::LIGHT_RED)
                } else {
                    (stack, Color32::DARK_RED)
                }
            }
        }
    }
}

/// Renders the duplicate fild finder page
pub fn duplicate_ui(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    dss: &mut DuplicateScannerUI,
    media_groups: Vec<MediaGroup>,
) {
    let is_scanning = dss.is_scanning();

    ui.add_space(30.0);

    // Path and FileDialog
    ui.horizontal(|ui| {
        let name_label = ui.strong("Path: ");
        ui.text_edit_singleline(&mut dss.path)
            .labelled_by(name_label.id);

        if ui
            .add_enabled(!is_scanning, egui::Button::new("\u{e613} Select Directory"))
            .clicked()
        {
            dss.file_dialog.select_directory();
        }
    });

    // Infofield
    let info = dss.messenger.info.lock().unwrap().to_string();
    ui.heading(info);
    ui.add_space(5.0);

    // Progressbar
    let f_items_max = dss.messenger.cntmax.lock().as_deref().unwrap().clone() as f32;
    let f_items_current = dss.messenger.cntcur.lock().as_deref().unwrap().clone() as f32;
    let progress = f_items_current / f_items_max;
    let progress_bar = egui::ProgressBar::new(progress).show_percentage();
    ui.add(progress_bar);

    // Scan / Abort Buttons
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!is_scanning, egui::Button::new("Start Scan"))
            .clicked()
        {
            dss.clear();
            let messenger = dss.messenger.clone();
            let path = dss.path.clone();

            dss.handle = Some(thread::spawn(move || {
                scan(Path::new(&path), ScanType::BINARY, media_groups, messenger);
            }));
        }

        if ui
            .add_enabled(is_scanning, egui::Button::new("Abort"))
            .clicked()
        {
            *dss.messenger.scanner_control.lock().unwrap() = Control::STOP;
        }

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.separator();
            ui.radio_value(
                &mut dss.view,
                ShowView::Scanned,
                format!("Scanned [{}]", dss.messenger.cntstd()),
            );
            ui.radio_value(
                &mut dss.view,
                ShowView::Errors,
                format!("Errors [{}]", dss.messenger.cnterr()),
            );
            ui.radio_value(
                &mut dss.view,
                ShowView::Duplicates,
                format!("Duplicates [{}]", dss.messenger.cntres()),
            );
        });
    });

    // Open FileDialog
    match dss.file_dialog.update(ctx).selected() {
        Some(path) => dss.path = path.to_str().unwrap_or("").to_owned(),
        _ => (),
    }
    ui.separator();

    // Scroll-Area LOG
    let text_style = TextStyle::Monospace;
    let row_height = ui.text_style_height(&text_style);
    let scroll_area = ScrollArea::vertical()
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .auto_shrink(false)
        .hscroll(true)
        .stick_to_bottom(true);

    let (stack, color) = dss.color(&ui);
    scroll_area.show_rows(ui, row_height, stack.len(), |ui, row_range| {
        for row in row_range {
            let msg = stack.get(row).unwrap();
            let rt = RichText::new(msg).color(color);
            ui.add(Label::new(rt).wrap_mode(TextWrapMode::Extend));
            if is_scanning && !ctx.has_requested_repaint() {
                ctx.request_repaint();
            }
        }
    });

    if is_scanning && !ctx.has_requested_repaint() {
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
