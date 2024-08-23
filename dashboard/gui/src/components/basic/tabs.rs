// Based on https://github.com/damus-io/egui-tabs

use eframe::egui::{vec2, Color32, CursorIcon, Layout, Direction, Sense};

pub struct Tabs {
    cols: u8,
    height: f32,
    sense: Sense,
    layout: Layout,
    selected_bg: Color32,
    selected_fg: Color32,
    hover_bg: Color32,
    hover_fg: Color32,
    bg: Color32,
    fg: Color32,
    stroke_bg: Color32,
    selected: u8,
    enabled: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TabState {
    ind: u8,
    hovered_tab: u8,
    selected_tab: u8,
}

impl TabState {
    pub fn is_hovered(&self) -> bool {
        self.hovered_tab == self.ind
    }

    pub fn is_selected(&self) -> bool {
        self.selected_tab == self.ind
    }

    pub fn hovered_tab(&self) -> u8 {
        self.hovered_tab
    }

    pub fn selected_tab(&self) -> u8 {
        self.selected_tab
    }

    pub fn index(&self) -> u8 {
        self.ind
    }
}

#[derive(Default, Debug)]
pub struct TabResponse<T> {
    inner: Vec<eframe::egui::InnerResponse<T>>,
    hovered: u8,
    selected: u8,
}

impl<T> TabResponse<T> {
    pub fn hovered(&self) -> u8 {
        self.hovered
    }

    pub fn selected(&self) -> u8 {
        self.selected
    }

    pub fn inner(self) -> Vec<eframe::egui::InnerResponse<T>> {
        self.inner
    }
}

impl Tabs {
    pub fn new(cols: u8, visuals: &eframe::egui::Visuals, enabled: bool) -> Self {
        Tabs {
            cols: cols.min(254), // Allow only 254 tabs;
            enabled,
            height: 20.0,
            sense: Sense::click(),
            layout: Layout::centered_and_justified(Direction::LeftToRight),
            selected_bg: visuals.selection.bg_fill,
            selected_fg: visuals.selection.stroke.color,
            hover_bg: visuals.widgets.hovered.bg_fill,
            hover_fg: visuals.widgets.hovered.fg_stroke.color,
            bg: visuals.faint_bg_color,
            fg: visuals.widgets.active.fg_stroke.color,
            stroke_bg: Color32::from_rgb(170, 170, 170),
            selected: 255,
        }
    }

    pub fn bg(mut self, bg: Color32) -> Self {
        self.bg = bg;
        self
    }

    pub fn fg(mut self, fg: Color32) -> Self {
        self.fg = fg;
        self
    }

    pub fn stroke_bg(mut self, stroke_bg: Color32) -> Self {
        self.stroke_bg = stroke_bg;
        self
    }

    pub fn hover_bg(mut self, bg_fill: Color32) -> Self {
        self.hover_bg = bg_fill;
        self
    }

    pub fn hover_fg(mut self, hover_fg: Color32) -> Self {
        self.hover_fg = hover_fg;
        self
    }

    pub fn selected_fg(mut self, selected_fg: Color32) -> Self {
        self.selected_fg = selected_fg;
        self
    }

    pub fn selected_bg(mut self, bg_fill: Color32) -> Self {
        self.selected_bg = bg_fill;
        self
    }

    /// The initial selection value
    pub fn selected(mut self, selected: u8) -> Self {
        self.selected = selected;
        self
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    /// The layout of the content in the cells
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn show<F, R>(&mut self, ui: &mut eframe::egui::Ui, add_tab: F) -> TabResponse<R>
    where
        F: Fn(&mut eframe::egui::Ui, TabState) -> R,
    {
        let mut inner = Vec::with_capacity(self.cols as usize);

        // Paint a stroke around the tab-group
        let mut rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(rect, 0.0, self.stroke_bg);

        rect.set_height(rect.height() - 1.0);
        rect.set_top(rect.top() + 1.0);
        let cell_width = rect.width() / self.cols as f32;
        rect.set_width(cell_width);

        let tabs_id = ui.id().with("tabs");
        let hover_id = tabs_id.with("hover");

        let mut hovered = 255;
        let mut selected = self.selected;

        for ind in 0..self.cols {
            let resp = ui.allocate_rect(rect, self.sense);

            // Save / Restore selected/hovered from temp store
            let selected_tab = if resp.clicked() {
                selected = ind;
                ui.ctx().data_mut(|d| d.insert_temp(tabs_id, ind));
                ind
            } else {
                ui.ctx()
                    .data(|d| d.get_temp(tabs_id))
                    .or(Some(self.selected))
                    .unwrap_or(255)
            };

            // Clear hover
            ui.data_mut(|data| data.remove::<u8>(hover_id));
            // Update hover if any
            let hovered_tab = if resp.hovered() {
                hovered = ind;
                ui.ctx().data_mut(|d| d.insert_temp(hover_id, ind));
                ind
            } else {
                ui.ctx().data(|d| d.get_temp(hover_id)).unwrap_or(255)
            };

            let tab_state = TabState {
                ind,
                selected_tab,
                hovered_tab,
            };

            // Paint the rectangle
            // preserve stroke line
            if ind == 0 { rect.set_left(rect.left() + 1.0) }

            let mut child_ui = ui.child_ui(rect, self.layout, None);

            if self.enabled {
                if tab_state.is_selected() {
                    selected = ind;
                    let mut r = rect.clone();

                    // paint stroke and round tab
                    r.set_top(r.top() - 3.0);
                    ui.painter().rect_stroke(r, 3.0, (1.0, ui.visuals().widgets.hovered.fg_stroke.color));
                    r.set_top(r.top() + 1.0);
                    r.set_bottom(r.bottom() + 1.0);
                    ui.painter().rect_filled(r, 3.0, self.selected_bg);

                    // paint lower rect without rounding
                    r.set_top(r.top() + 4.0);
                    ui.painter().rect_filled(r, 0.0, self.selected_bg);
                } else if tab_state.is_hovered() {
                    hovered = ind;
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    ui.painter().rect_filled(rect, 0.0, self.hover_bg);
                } else {
                    ui.painter().rect_filled(rect, 0.0, self.bg);
                }

                // set foreground colors
                if tab_state.is_selected() {
                    child_ui.style_mut().visuals.override_text_color = Some(self.selected_fg);
                } else if tab_state.is_hovered() {
                    child_ui.style_mut().visuals.override_text_color = Some(self.hover_fg);
                } else {
                    child_ui.style_mut().visuals.override_text_color = Some(self.fg);
                }
            } // if enabled

            // Call code parent and run show() method (i.e. add Label
            let user_value = add_tab(&mut child_ui, tab_state);
            inner.push(eframe::egui::InnerResponse::new(user_value, resp));

            rect = rect.translate(vec2(cell_width, 0.0))
        }

        TabResponse {
            selected,
            hovered,
            inner,
        }
    }
}
