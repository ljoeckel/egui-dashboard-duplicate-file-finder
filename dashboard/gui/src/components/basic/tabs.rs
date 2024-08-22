// Based on https://github.com/damus-io/egui-tabs

use eframe::egui::{vec2, Color32, CursorIcon, Layout, Sense};

pub struct Tabs {
    cols: i32,
    height: f32,
    sense: Sense,
    layout: Layout,
    selected_bg: Color32,
    selected_fg: Color32,
    hover_bg: Color32,
    hover_fg: Color32,
    bg: Color32,
    fg: Color32,
    selected: Option<i32>,
    enabled: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TabState {
    ind: i32,
    hovered_tab: i32,
    selected_tab: i32,
}

impl TabState {
    pub fn is_hovered(&self) -> bool {
        self.hovered_tab == self.ind
    }

    pub fn is_selected(&self) -> bool {
        self.selected_tab == self.ind
    }

    pub fn hovered_tab(&self) -> Option<i32> {
        if self.hovered_tab < 0 {
            None
        } else {
            Some(self.hovered_tab)
        }
    }

    pub fn selected_tab(&self) -> Option<i32> {
        if self.selected_tab < 0 {
            None
        } else {
            Some(self.selected_tab)
        }
    }

    pub fn index(&self) -> i32 {
        self.ind
    }
}

#[derive(Default, Debug)]
pub struct TabResponse<T> {
    inner: Vec<eframe::egui::InnerResponse<T>>,
    hovered: Option<i32>,
    selected: Option<i32>,
}

impl<T> TabResponse<T> {
    pub fn hovered(&self) -> Option<i32> {
        self.hovered
    }

    pub fn selected(&self) -> Option<i32> {
        self.selected
    }

    pub fn inner(self) -> Vec<eframe::egui::InnerResponse<T>> {
        self.inner
    }
}

impl Tabs {
    pub fn new(cols: i32, visuals: &eframe::egui::Visuals, enabled: bool) -> Self {
        Tabs {
            cols,
            enabled,
            height: 20.0,
            sense: Sense::click(),
            layout: Layout::default(),
            selected_bg: visuals.selection.bg_fill,
            selected_fg: visuals.selection.stroke.color,
            hover_bg: visuals.widgets.hovered.bg_fill,
            hover_fg: visuals.widgets.hovered.fg_stroke.color,
            bg: visuals.widgets.active.bg_fill,
            fg: Color32::BLACK,
            selected: None,
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
    pub fn selected(mut self, selected: i32) -> Self {
        self.selected = Some(selected);
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

        if self.cols == 0 {
            return TabResponse {
                selected: None,
                hovered: None,
                inner,
            };
        }

        // Paint a stroke around the tab-group
        let mut rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(170, 170, 170));

        rect.set_height(rect.height() - 1.0);
        rect.set_top(rect.top() + 1.0);
        let cell_width = rect.width() / self.cols as f32;
        rect.set_width(cell_width);

        let tabs_id = ui.id().with("tabs");
        let hover_id = tabs_id.with("hover");
        let mut any_hover = false;

        let mut selected: Option<i32> = self.selected;
        let mut hovered: Option<i32> = None;

        for ind in 0..self.cols {
            let resp = ui.allocate_rect(rect, self.sense);

            let selected_tab = if resp.clicked() {
                selected = Some(ind);
                ui.ctx().data_mut(|d| d.insert_temp(tabs_id, ind));
                ind
            } else {
                ui.ctx()
                    .data(|d| d.get_temp::<i32>(tabs_id))
                    .or(self.selected)
                    .unwrap_or(-1)
            };

            let hovered_tab = if resp.hovered() {
                any_hover = true;
                hovered = Some(ind);
                ui.ctx().data_mut(|d| d.insert_temp(hover_id, ind));
                ind
            } else {
                ui.ctx().data(|d| d.get_temp::<i32>(hover_id)).unwrap_or(-1)
            };

            let tab_state = TabState {
                ind,
                selected_tab,
                hovered_tab,
            };

            // preserve stroke line
            if ind == 0 { rect.set_left(rect.left() + 1.0) }

            if tab_state.is_selected() {
                selected = Some(ind);
                let mut r = rect.clone();

                if self.enabled {
                    // paint stroke and round tab
                    r.set_top(r.top() - 3.0);
                    ui.painter().rect_stroke(r, 3.0, (1.0, ui.visuals().widgets.hovered.fg_stroke.color));
                    r.set_top(r.top() + 1.0);
                    r.set_bottom(r.bottom() + 1.0);
                    ui.painter().rect_filled(r, 3.0, self.selected_bg);

                    // paint lower rect without rounding
                    r.set_top(r.top() + 4.0);
                    ui.painter().rect_filled(r, 0.0, self.selected_bg);
                }

            } else if tab_state.is_hovered() {
                if self.enabled {
                    hovered = Some(ind);
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    ui.painter().rect_filled(rect, 0.0, self.hover_bg);
                }
            } else {
                if self.enabled {
                    ui.painter().rect_filled(rect, 0.0, self.bg);
                }
            }

            // set foreground colors
            let mut child_ui = ui.child_ui(rect, self.layout, None);

            if self.enabled {
                if tab_state.is_selected() {
                    child_ui.style_mut().visuals.override_text_color = Some(self.selected_fg);
                } else if tab_state.is_hovered() {
                    child_ui.style_mut().visuals.override_text_color = Some(self.hover_fg);
                } else {
                    child_ui.style_mut().visuals.override_text_color = Some(self.fg);
                }
            }

            let user_value = add_tab(&mut child_ui, tab_state);
            inner.push(eframe::egui::InnerResponse::new(user_value, resp));

            rect = rect.translate(vec2(cell_width, 0.0))
        }

        if !any_hover {
            ui.data_mut(|data| data.remove::<i32>(hover_id));
            hovered = None;
        }

        TabResponse {
            selected,
            hovered,
            inner,
        }
    }
}
