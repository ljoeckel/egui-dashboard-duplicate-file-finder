use eframe::egui;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Add the font
    let font_name = "BlexMono";
    fonts.font_data.insert(
        font_name.to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/IBMPlexMono/BlexMonoNerdFont-Regular.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        //.push(font_name.to_owned())
        .insert(0, font_name.to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, font_name.to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub fn list_font_families() {
    let fonts = egui::FontDefinitions::default();

    for family in fonts.families.iter() {
        println!("font={:?}", family);
    }
}
