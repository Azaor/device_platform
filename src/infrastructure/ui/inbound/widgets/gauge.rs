use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

pub fn gauge(ui: &mut Ui, value: u64, min: u64, max: u64, size: Vec2) -> Response {
    // Reserve l'espace
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter_at(rect);

    // Fond de la jauge
    painter.rect_filled(rect, 4.0, Color32::DARK_GRAY);

    // Calcul du pourcentage rempli
    let clamped_temp = value.clamp(min, max);
    let t = (clamped_temp as f32 - min as f32) / (max as f32 - min as f32);

    // Hauteur du niveau de température
    let mut filled_height = t * rect.height();
    let mut filled_width = rect.width();
    if size.y < size.x {
        filled_height = rect.height();
        filled_width = t * rect.width();
    }
    // Rectangle rempli (du bas vers le haut)
    let filled_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.bottom() - filled_height),
        Pos2::new(rect.left()+filled_width, rect.bottom()),
    );

    let fill_color = egui::Color32::from_rgb(
        0,
        0,
        255,
    );

    painter.rect_filled(filled_rect, 4.0, fill_color);

    // Affiche la température en texte
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{:.1}", value),
        egui::TextStyle::Body.resolve(ui.style()),
        Color32::WHITE,
    );

    response
}
