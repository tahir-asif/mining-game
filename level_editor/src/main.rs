use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let levels_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("levels.ron"));

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 550.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Level Editor",
        native_options,
        Box::new(move |_cc| Ok(Box::new(level_editor::LevelEditor::new(levels_path)))),
    )
}
