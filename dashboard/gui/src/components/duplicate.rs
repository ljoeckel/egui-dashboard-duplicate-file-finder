use crate::scanner::mediatype::{MediaGroup, ScanType};
use crate::scanner::messenger::Messenger;
use crate::scanner::scanner::scan;

use std::{
    fs::remove_file,
    path::Path,
    sync::MutexGuard,
    thread::{self},
    time::Duration,
    vec::Vec,
};
use std::rc::Rc;
use eframe::egui;
use eframe::egui::{Color32, Label, RichText, ScrollArea, TextStyle};
use egui::scroll_area::ScrollBarVisibility;
use egui::{epaint::text::TextWrapMode, Ui};

use egui_aesthetix::Aesthetix;
use egui_file_dialog::FileDialog;

use crate::components::notifications::NotificationBar;
use crate::components::{basic, duplicates_table};
use basic::Tabs;

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
    pub fn from(tab_idx: u8) -> Self {
        match tab_idx {
            0 => ShowTab::Scanned,
            1 => ShowTab::Errors,
            _ => ShowTab::Duplicates,
        }
    }
    pub fn index(&self) -> usize {
        *self  as usize
    }
}

pub struct DuplicateScannerUI {
    current_tab: ShowTab,
    path: String,
    file_dialog: FileDialog,
    messenger: Messenger,
    scanning: bool,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl DuplicateScannerUI {
    pub fn new() -> Self {
        Self {
            current_tab: ShowTab::Scanned,
            path: String::new(),
            file_dialog: FileDialog::new(),
            messenger: Messenger::new(),
            scanning: false,
            handle: None,
        }
    }

    pub fn set_tab(&mut self, tab: u8) {
        self.current_tab = ShowTab::from(tab);
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

    fn get_tab_data(&self) -> (MutexGuard<Vec<String>>, MutexGuard<Vec<bool>>) {
        let stack: MutexGuard<Vec<String>>;
        match self.current_tab {
            ShowTab::Scanned => {
                stack = self.messenger.stdlog();
            }
            ShowTab::Duplicates => {
                stack = self.messenger.reslog();
            }
            ShowTab::Errors => {
                stack = self.messenger.errlog();
            }
        }
        (stack, self.messenger.checked())
    }

    fn get_tab_color(&self, ui: &Ui) -> Color32 {
        let color_idx = self.current_tab.index();
        let mut dark_idx = 0;
        if ui.visuals().dark_mode {
            dark_idx = 1;
        }
        TAB_COLORS[color_idx][dark_idx]
    }

}


/// Renders the duplicate fild finder page
pub fn duplicate_ui(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    dss: &mut DuplicateScannerUI,
    media_groups: Vec<MediaGroup>,
    notification_bar: &mut NotificationBar,
    active_theme: &Rc<dyn Aesthetix>,
    zoom_factor: f32,
) {
    let is_scanning = dss.is_scanning();
    let have_results = dss.have_results();

    // Update the NotificationBar
    notification_bar.info(&dss.messenger.info());

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
    ui.add_space(12.0);
    ui.separator();

    // Update data for the ProgressBar
    notification_bar.set_progress(dss.messenger.progress(), "");

    // Scan / Abort Buttons
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!is_scanning, egui::Button::new("Start Scan"))
            .clicked()
        {
            dss.clear();
            notification_bar.clear();

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
            dss.messenger.stop();
        }

        ui.add_space(5.0);

        let res = Tabs::new(3, &ui.visuals(), have_results)
        .height(ui.text_style_height(&TextStyle::Button) * 1.4)
            .selected(0)
            .selected_bg(Color32::from_rgb(206,231,218))
            .selected_fg(Color32::BLACK)
            .hover_bg(Color32::from_rgb(218,207,181))
            .hover_fg(Color32::BLACK)
            .bg(Color32::from_rgb(226,221,213))
            .fg(Color32::DARK_GRAY)
            .show(ui, |ui, state| {
                // the add_tab method
                let tab_headers = vec!["Scanned", "Problems", "Duplicates"];
                let tab_header = tab_headers[state.index() as usize];
                let cnt;
                if state.index() == 0 {
                    cnt = dss.messenger.cntstd();
                } else if state.index() == 1 {
                    cnt = dss.messenger.cnterr();
                } else if state.index() == 2 {
                    cnt = dss.messenger.cntres();
                } else {
                    cnt = 0;
                }
                ui.add_enabled(have_results, egui::Label::new(format!("{} [{}]", tab_header, cnt)).selectable(false));
            });

        dss.set_tab(res.selected()); // Set ShowTab::xxx
    });

    // Open FileDialog
    match dss.file_dialog.update(ctx).selected() {
        Some(path) => dss.path = path.to_str().unwrap_or("").to_owned(),
        _ => (),
    }

    ui.separator();

    // Scroll-Area LOG
    //let text_style = TextStyle::Monospace;
    let row_height = ui.text_style_height(&TextStyle::Monospace);
    let scroll_area = ScrollArea::vertical()
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
        .auto_shrink(false)
        .hscroll(true)
        .stick_to_bottom(true);

    let (mut stack, checked) = dss.get_tab_data();
    let color = dss.get_tab_color(&ui);

    if dss.current_tab == ShowTab::Duplicates {
        ui.vertical(|vert| {
            duplicates_table::mediatable(vert, active_theme, &mut stack, checked, zoom_factor);
        }); // vert
    } else {
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
    }

    if !ctx.has_requested_repaint() {
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
