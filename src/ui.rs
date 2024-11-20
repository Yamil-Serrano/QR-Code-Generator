// Import the GUI library we're using
use eframe::egui;

// This function sets up how our app looks (colors, etc)
pub fn apply_style(ctx: &egui::Context) {
    // Get the current style settings so we can modify them
    let mut style = (*ctx.style()).clone();
    
    // Change colors to make it look nice:
    style.visuals.window_fill = egui::Color32::WHITE;             // White window background
    style.visuals.panel_fill = egui::Color32::WHITE;              // White panel background
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(230, 230, 230);  // Light gray for text boxes
    
    // Apply our modified style
    ctx.set_style(style);
}

// This is our main rendering function that draws everything
pub fn render(app: &mut crate::app::QrGeneratorApp, ctx: &egui::Context) {
    // Create the main panel that holds everything
    egui::CentralPanel::default().show(ctx, |ui| {
        // Make everything centered
        ui.vertical_centered(|ui| {
            // Add the title at the top
            ui.heading("QR Code Generator");
            ui.add_space(20.0);  // Add some space below the title
            
            // Add the label above the text box
            ui.label("Insert your link here:");
            ui.add_space(5.0);  // Small space after label

            // Create the text box where users type their URL
            let text_edit = egui::TextEdit::singleline(&mut app.link)
                .desired_width(ui.available_width() * 0.8)  // Make it 80% of the window width
                .text_color(egui::Color32::GRAY);          // Gray text color
            
            // Add the text box to our UI
            ui.add(text_edit);
            ui.add_space(20.0);  // Space after text box

            // Draw the QR code display area
            render_qr_display(app, ui);
            ui.add_space(20.0);  // Space after QR code
            
            // Add the Generate button and handle clicks
            if ui.add(
                egui::Button::new(egui::RichText::new("Generate").color(egui::Color32::GRAY))
                    .fill(egui::Color32::from_rgb(230, 230, 230))  // Light gray button
            ).clicked() {
                // When clicked, start generating the QR code
                app.generate_qr_code();
            }
        });
    });
}

// This function handles drawing the QR code (or placeholder if none exists)
fn render_qr_display(app: &crate::app::QrGeneratorApp, ui: &mut egui::Ui) {
    // Set the size for our QR code display (256x256 pixels)
    let qr_size = egui::vec2(256.0, 256.0);
    
    // Reserve space in the UI for our QR code
    let (rect, _) = ui.allocate_exact_size(qr_size, egui::Sense::hover());
    
    // Check if we have a QR code to display
    if let Some(texture) = &app.qr_texture {
        // If we have a QR code, draw it
        ui.painter().image(
            texture.id(),
            rect,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(1.0)),
            egui::Color32::WHITE,
        );
    } else {
        // If no QR code yet, draw a placeholder
        // Draw the border
        ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        // Draw the placeholder text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "QR Code will appear here",
            egui::FontId::proportional(14.0),
            egui::Color32::GRAY,
        );
    }
}

// This function converts raw image data into a texture we can display
pub fn load_texture(ctx: &egui::Context, image_data: &[u8]) -> egui::TextureHandle {
    // Load the image from the raw bytes
    let image = image::load_from_memory(image_data).expect("Failed to load image");
    
    // Get the image dimensions
    let size = [image.width() as _, image.height() as _];
    
    // Convert the image to RGBA format
    let image_buffer = image.to_rgba8();
    
    // Get the raw pixel data
    let pixels = image_buffer.as_flat_samples();
    
    // Create and return a texture from our image data
    ctx.load_texture(
        "qr-code",  // Name for our texture
        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
        egui::TextureOptions::default()
    )
}