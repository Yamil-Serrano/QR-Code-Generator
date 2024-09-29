#![windows_subsystem = "windows"] //prevents the console to open


use eframe::egui; // Import the egui module from the eframe crate for GUI components
use qrcode_generator::to_png_to_vec; // Import the function to generate QR codes in PNG format
use std::sync::mpsc; // Import multi-producer, single-consumer channels for thread communication
use std::thread; // Import threading capabilities

// Define the main application structure for the QR Code Generator
struct QrGeneratorApp {
    link: String, // Stores the user-provided link to generate the QR code
    qr_texture: Option<egui::TextureHandle>, // Holds the texture for the generated QR code image
    tx: mpsc::Sender<Vec<u8>>, // Sender channel to send QR code data from the worker thread
    rx: mpsc::Receiver<Vec<u8>>, // Receiver channel to receive QR code data in the main thread
}

// Implement the Default trait for QrGeneratorApp to provide default values
impl Default for QrGeneratorApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel(); // Create a new channel for communication
        Self {
            link: String::new(), // Initialize the link as an empty string
            qr_texture: None, // No QR code texture initially
            tx,
            rx,
        }
    }
}

// Implement the eframe::App trait to define the application's behavior
impl eframe::App for QrGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Configure a white background for the application window
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::WHITE; // Set window background color to white
        style.visuals.panel_fill = egui::Color32::WHITE; // Set panel background color to white
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(230, 230, 230); // Set text box color to light gray

        // Apply the customized style to the context
        ctx.set_style(style);

        // Define the central panel of the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("QR Code Generator"); // Display the application heading

                ui.add_space(20.0); // Add vertical space

                ui.label("Insert your link here:"); // Prompt the user to enter a link

                ui.add_space(5.0); // Add a small space between the label and the text input

                // Create a single-line text input for the user to enter the link
                let text_edit = egui::TextEdit::singleline(&mut self.link)
                    .desired_width(ui.available_width() * 0.8) // Set the width to 80% of available space
                    .text_color(egui::Color32::GRAY); // Set the text color to gray

                ui.add(text_edit); // Add the text input to the UI

                ui.add_space(20.0); // Add vertical space

                // Define the size for the QR code display area
                let qr_size = egui::vec2(256.0, 256.0);
                // Allocate space for the QR code image
                let (rect, _) = ui.allocate_exact_size(qr_size, egui::Sense::hover());
                if let Some(texture) = &self.qr_texture {
                    // If a QR code texture exists, display it
                    ui.painter().image(
                        texture.id(),
                        rect,
                        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(1.0)),
                        egui::Color32::WHITE,
                    );
                } else {
                    // If no QR code is generated yet, display a placeholder
                    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "QR Code will appear here",
                        egui::FontId::proportional(14.0),
                        egui::Color32::GRAY,
                    );
                }

                ui.add_space(20.0); // Add vertical space

                // Create a "Generate" button with custom colors
                if ui.add(
                    egui::Button::new(egui::RichText::new("Generate").color(egui::Color32::GRAY)) // Button text color
                        .fill(egui::Color32::from_rgb(230, 230, 230)) // Button background color (light gray)
                ).clicked() {
                    let link = self.link.clone(); // Clone the link to move into the thread
                    let tx = self.tx.clone(); // Clone the sender to move into the thread
                    thread::spawn(move || {
                        // Spawn a new thread to generate the QR code
                        if let Ok(qr_code) = to_png_to_vec(&link, qrcode_generator::QrCodeEcc::Low, 256) {
                            // If QR code generation is successful, send the PNG data through the channel
                            let _ = tx.send(qr_code);
                        }
                    });
                }
            });
        });

        // Check if any QR code data has been received from the worker thread
        if let Ok(qr_data) = self.rx.try_recv() {
            self.qr_texture = Some(load_texture(ctx, &qr_data));
        }
    }
}

// Function to load image data into an egui texture
fn load_texture(ctx: &egui::Context, image_data: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(image_data).expect("Failed to load image"); // Load image from byte data
    let size = [image.width() as _, image.height() as _]; // Get image dimensions
    let image_buffer = image.to_rgba8(); // Convert the image to RGBA8 format
    let pixels = image_buffer.as_flat_samples(); // Flatten the image buffer for texture creation
    ctx.load_texture(
        "qr-code", // Name identifier for the texture
        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()), // Create a color image from RGBA data
        egui::TextureOptions::default() // Use default texture options
    )
}

// The main function to run the application
fn main() -> eframe::Result<()> {
    let window_size = egui::vec2(400.0, 450.0); // Define the initial window size
    let options = eframe::NativeOptions {
        initial_window_size: Some(window_size), // Set the initial window size
        min_window_size: Some(window_size), // Set the minimum window size
        max_window_size: Some(window_size), // Set the maximum window size
        resizable: false, // Make the window non-resizable
        icon_data: load_icon(), // Load the application icon
        ..Default::default() // Use default settings for other options
    };
    eframe::run_native(
        "QR Code Generator", // Application title
        options,
        Box::new(|_cc| Box::new(QrGeneratorApp::default())), // Initialize the application with default settings
    )
}

// Function to load the application icon from a file
fn load_icon() -> Option<eframe::IconData> {
    // Incluir el icono embebido en el binario
    let icon_bytes = include_bytes!("../assets/qr-code.ico");
    let icon = image::load_from_memory(icon_bytes).ok()?.into_rgba8();
    let (width, height) = icon.dimensions();
    let rgba = icon.into_raw();

    Some(eframe::IconData {
        rgba,
        width,
        height,
    })
}
