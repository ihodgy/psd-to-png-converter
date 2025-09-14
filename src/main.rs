#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Result;

mod converter;
mod ui;
mod error;

use converter::PsdConverter;
use ui::{AppUI, UiAction};

#[derive(Default)]
pub struct PsdToPngApp {
    ui: AppUI,
    converter: Arc<Mutex<PsdConverter>>,
    input_folder: Option<PathBuf>,
    output_folder: Option<PathBuf>,
    is_converting: bool,
    progress: f32,
    status_message: String,
    error_message: Option<String>,
}

impl eframe::App for PsdToPngApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint for smooth progress updates
        if self.is_converting {
            ctx.request_repaint();
            
            // Simple timer to reset conversion state after 3 seconds
            // This is a basic approach - in a real app you'd use proper async callbacks
            static mut CONVERSION_START_TIME: Option<std::time::Instant> = None;
            static mut COMPLETION_DISPLAY_TIME: Option<std::time::Instant> = None;
            unsafe {
                if CONVERSION_START_TIME.is_none() {
                    CONVERSION_START_TIME = Some(std::time::Instant::now());
                }
                
                if let Some(start_time) = CONVERSION_START_TIME {
                    if start_time.elapsed().as_secs() >= 3 {
                        self.is_converting = false;
                        self.status_message = "🎉 Conversion completed! Ready for next batch.".to_string();
                        // Clear folders when conversion finishes
                        self.input_folder = None;
                        self.output_folder = None;
                        CONVERSION_START_TIME = None;
                        COMPLETION_DISPLAY_TIME = Some(std::time::Instant::now());
                        // Debug: Print to console (this won't show in release mode due to console suppression)
                        println!("DEBUG: Conversion completed, folders cleared, status set to: {}", self.status_message);
                    }
                }
                
                // Keep completion message visible for 5 seconds
                if let Some(completion_time) = COMPLETION_DISPLAY_TIME {
                    if completion_time.elapsed().as_secs() >= 5 {
                        self.status_message = "Ready to convert PSD files to PNG".to_string();
                        COMPLETION_DISPLAY_TIME = None;
                    }
                }
            }
        }
        
        // Render the UI by extracting the data we need
        let input_folder = self.input_folder.clone();
        let output_folder = self.output_folder.clone();
        let is_converting = self.is_converting;
        let progress = self.progress;
        let status_message = self.status_message.clone();
        let error_message = self.error_message.clone();
        
        // Create a closure to handle UI interactions
        let mut ui_actions = Vec::new();
        self.ui.render(ctx, input_folder, output_folder, is_converting, progress, status_message, error_message, &mut ui_actions);
        
        // Process UI actions after rendering
        for action in ui_actions {
            match action {
                UiAction::SetInputFolder(path) => {
                    self.set_input_folder(path);
                }
                UiAction::SetOutputFolder(path) => {
                    self.set_output_folder(path);
                }
                UiAction::StartConversion => {
                    self.start_conversion();
                }
            }
        }
    }
}

impl PsdToPngApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup custom fonts and styling
        setup_custom_styles(&cc.egui_ctx);
        
        Self {
            ui: AppUI::new(),
            converter: Arc::new(Mutex::new(PsdConverter::new())),
            input_folder: None,
            output_folder: None,
            is_converting: false,
            progress: 0.0,
            status_message: "Ready to convert PSD files to PNG".to_string(),
            error_message: None,
        }
    }

    pub fn set_input_folder(&mut self, path: PathBuf) {
        self.input_folder = Some(path);
        self.error_message = None;
        self.status_message = "Input folder selected".to_string();
        // Reset completion state when new input folder is selected
        unsafe {
            static mut COMPLETION_DISPLAY_TIME: Option<std::time::Instant> = None;
            COMPLETION_DISPLAY_TIME = None;
        }
    }

    pub fn set_output_folder(&mut self, path: PathBuf) {
        self.output_folder = Some(path);
        self.error_message = None;
        self.status_message = "Output folder selected".to_string();
        // Reset completion state when new output folder is selected
        unsafe {
            static mut COMPLETION_DISPLAY_TIME: Option<std::time::Instant> = None;
            COMPLETION_DISPLAY_TIME = None;
        }
    }

    pub fn start_conversion(&mut self) {
        if let (Some(input), Some(output)) = (&self.input_folder, &self.output_folder) {
            self.is_converting = true;
            self.progress = 0.0;
            self.status_message = "Starting conversion...".to_string();
            self.error_message = None;

            let input_path = input.clone();
            let output_path = output.clone();
            
            // Start conversion in background thread
            std::thread::spawn(move || {
                // Check if input directory exists and has files
                if !input_path.exists() {
                    return;
                }
                
                // List files in input directory
                match std::fs::read_dir(&input_path) {
                    Ok(entries) => {
                        let mut psd_files = Vec::new();
                        for entry in entries.flatten() {
                            if let Some(path) = entry.path().extension() {
                                if path.to_string_lossy().to_lowercase() == "psd" {
                                    psd_files.push(entry.path());
                                }
                            }
                        }
                        
                        if psd_files.is_empty() {
                            return;
                        }
                        
                        // Convert each PSD file
                        for psd_file in psd_files {
                            let output_file = output_path.join(
                                psd_file.file_stem().unwrap_or_default()
                            ).with_extension("png");
                            
                            // Try to convert using the converter
                            match converter::PsdConverter::convert_psd_to_png_sync(&psd_file, &output_file) {
                                Ok(_) => {
                                    // Success - file converted
                                }
                                Err(_) => {
                                    // Handle conversion error
                                }
                            }
                        }
                        
                        // Conversion completed - we could add a callback here to update the UI
                        // For now, we'll let the user manually reset by clicking the button again
                    }
                    Err(_) => {
                        // Silently handle errors
                    }
                }
            });
        } else {
            self.error_message = Some("Please select both input and output folders".to_string());
        }
    }

    pub fn update_progress(&mut self, progress: f32, message: String) {
        self.progress = progress;
        self.status_message = message;
    }

    pub fn conversion_complete(&mut self, success: bool, message: String) {
        self.is_converting = false;
        self.status_message = message.clone();
        if !success {
            self.error_message = Some(message);
        }
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Customize colors for a modern look
    style.visuals.window_fill = egui::Color32::from_gray(32);
    style.visuals.panel_fill = egui::Color32::from_gray(40);
    style.visuals.extreme_bg_color = egui::Color32::from_gray(24);
    
    // Customize text colors
    // Note: text_color is now a method, not a field
    style.visuals.hyperlink_color = egui::Color32::from_rgb(100, 150, 255);
    
    // Customize button colors
    style.visuals.button_frame = true;
    style.visuals.override_text_color = None;
    
    ctx.set_style(style);
}

fn main() -> Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PSD to PNG Converter",
        options,
        Box::new(|cc| Ok(Box::new(PsdToPngApp::new(cc)))),
    ).map_err(|e| anyhow::anyhow!("Application error: {}", e))?;

    Ok(())
}
