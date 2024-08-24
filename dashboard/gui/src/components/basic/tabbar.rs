use eframe::egui::{Label, TextWrapMode, RichText, Widget, vec2, Color32, CursorIcon, Layout, Direction, Sense, Ui, Response};

pub struct TabBar<'a> {
    selected: &'a mut usize,
    cols: Vec<String>,
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
}
impl<'a> TabBar<'a> {
    pub fn new(cols: Vec<String>, selected: &'a mut usize, visuals: &eframe::egui::Visuals) -> Self {
        TabBar {
            cols,
            selected,
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
}


impl<'a> Widget for TabBar<'a>
{
    fn ui(self, ui: &mut Ui) -> Response {
        // Paint a stroke around the tab-group
        let mut rect = ui.available_rect_before_wrap();

        let mut response = ui.allocate_rect(rect, self.sense);

        ui.painter().rect_filled(rect, 0.0, self.stroke_bg);
        rect.set_height(rect.height() - 1.0);
        rect.set_top(rect.top() + 1.0);
        let cell_width = rect.width() / self.cols.len() as f32;
        rect.set_width(cell_width);

        let mut fg_color: Color32;
        for (ind, header) in self.cols.iter().enumerate() {
            // Paint the rectangle while preserving the stroke lines
            if ind == 0 { rect.set_left(rect.left() + 1.0) }
            if ind == self.cols.len() - 1 {
                rect.set_width(rect.width() - 1.0);
            }

            let mut child_ui = ui.child_ui(rect, self.layout, None);

            let resp = ui.allocate_rect(rect, self.sense);
            let clicked = resp.clicked();
            let hovered = resp.hovered();

            if clicked {
                *self.selected = ind;
                response = resp.clone();
            }

            if *self.selected == ind {
                let mut r = rect.clone();

                // paint stroke and round tab
                r.set_top(r.top() - 3.0);
                ui.painter().rect_stroke(r, 3.0, (1.0, ui.visuals().widgets.hovered.fg_stroke.color));
                //r.set_top(r.top() + 1.0);
                r.set_bottom(r.bottom() + 1.0);
                ui.painter().rect_filled(r, 3.0, self.selected_bg);

                // paint lower rect without rounding
                r.set_top(r.top() + 4.0);
                ui.painter().rect_filled(r, 0.0, self.selected_bg);
                fg_color = self.selected_fg;
            } else if hovered {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                ui.painter().rect_filled(rect, 0.0, self.hover_bg);
                fg_color = self.hover_fg;
                response = resp.clone();
            } else {
                ui.painter().rect_filled(rect, 0.0, self.bg);
                fg_color = self.fg;
            };

            let rt = RichText::new(header).color(fg_color);
            child_ui.add(Label::new(rt).wrap_mode(TextWrapMode::Wrap).selectable(false));
            rect = rect.translate(vec2(cell_width, 0.0));
        }
        response
    }
}

