// Import the stuff we need from other crates
use eframe::egui;                     // For GUI components
use qrcode_generator::to_png_to_vec;  // For making QR codes
use std::sync::mpsc;                  // For sending messages between threads
use std::thread;                      // For creating new threads

// This is our main app struct that holds all our important data
pub struct QrGeneratorApp {
    pub link: String,                                 // Stores the URL the user types in
    pub qr_texture: Option<egui::TextureHandle>,      // Holds the QR code image (if we have one)
    tx: mpsc::Sender<Vec<u8>>,                       // For sending QR code data from worker thread
    rx: mpsc::Receiver<Vec<u8>>,                     // For receiving QR code data in main thread
}

// This tells Rust how to create a new instance of our app with default values
impl Default for QrGeneratorApp {
    fn default() -> Self {
        // Create a channel for sending data between threads
        let (tx, rx) = mpsc::channel();
        
        // Return a new app instance with empty/default values
        Self {
            link: String::new(),          // Empty string for the URL
            qr_texture: None,             // No QR code image yet
            tx,                           // Sender part of our channel
            rx,                           // Receiver part of our channel
        }
    }
}

// Functions that are specific to our QrGeneratorApp
impl QrGeneratorApp {
    // This function starts the QR code generation in a separate thread
    pub fn generate_qr_code(&self) {
        // Clone these because we need to move them into the new thread
        let link = self.link.clone();
        let tx = self.tx.clone();
        
        // Start a new thread to generate the QR code
        thread::spawn(move || {
            // Try to generate a QR code from our link
            if let Ok(qr_code) = to_png_to_vec(&link, qrcode_generator::QrCodeEcc::Low, 256) {
                // If it worked, send the QR code data back to the main thread
                let _ = tx.send(qr_code);
            }
        });
    }

    // This function checks if we have a new QR code ready to display
    pub fn check_for_qr_update(&mut self, ctx: &egui::Context) {
        // Try to receive QR code data from the worker thread
        if let Ok(qr_data) = self.rx.try_recv() {
            // If we got data, convert it into a texture we can display
            self.qr_texture = Some(crate::ui::load_texture(ctx, &qr_data));
        }
    }
}

// This is where we implement what our app does each frame
impl eframe::App for QrGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply our custom style to make it look nice
        crate::ui::apply_style(ctx);
        
        // Draw all our UI components
        crate::ui::render(self, ctx);
        
        // Check if we have a new QR code to display
        self.check_for_qr_update(ctx);
    }
}