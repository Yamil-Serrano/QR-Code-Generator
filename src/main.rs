// Import our local modules that contain the app logic and UI components
mod app;
mod ui;

// This tells Windows not to show a console window when running the app but is on by default
//#[cfg(windows)]
//#[windows_subsystem = "windows"]

// Main function that starts our application. Returns a Result type from eframe
fn main() -> eframe::Result<()> {
    // Define the size of our window (width: 400, height: 450)
    let window_size = eframe::egui::vec2(400.0, 450.0);
    
    // Create window options for our application
    let options = eframe::NativeOptions {
        initial_window_size: Some(window_size),    // Set starting window size
        min_window_size: Some(window_size),        // User can't make window smaller than this
        max_window_size: Some(window_size),        // User can't make window bigger than this
        resizable: false,                          // Prevent window resizing
        icon_data: load_icon(),                    // Load our app icon
        ..Default::default()                       // Use defaults for all other options
    };
    
    // Start running our native application with these settings:
    // - "QR Code Generator" is the window title
    // - options contains our window settings
    // - The last line creates a new instance of our app
    eframe::run_native(
        "QR Code Generator",
        options,
        Box::new(|_cc| Box::new(app::QrGeneratorApp::default())),
    )
}

// Function to load our application icon from a file
fn load_icon() -> Option<eframe::IconData> {
    // Read the icon file bytes that are embedded in our executable
    let icon_bytes = include_bytes!("../assets/qr-code.ico");
    
    // Try to load the icon from memory and convert it to RGBA format
    let icon = image::load_from_memory(icon_bytes).ok()?.into_rgba8();
    
    // Get the width and height of our icon
    let (width, height) = icon.dimensions();
    
    // Convert the icon into raw bytes
    let rgba = icon.into_raw();

    // Return the icon data in the format eframe expects
    Some(eframe::IconData {
        rgba,
        width,
        height,
    })
}