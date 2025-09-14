use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Debug)]
pub enum UiAction {
    SetInputFolder(PathBuf),
    SetOutputFolder(PathBuf),
    StartConversion,
}

#[derive(Default)]
pub struct AppUI {
    drag_state: DragState,
}

#[derive(Default)]
pub struct DragState {
    hovered: bool,
    drag_over: bool,
}

impl AppUI {
    pub fn new() -> Self {
        Self {
            drag_state: DragState::default(),
        }
    }

    pub fn render(
        &mut self, 
        ctx: &egui::Context, 
        input_folder: Option<PathBuf>,
        output_folder: Option<PathBuf>,
        is_converting: bool,
        progress: f32,
        status_message: String,
        error_message: Option<String>,
        actions: &mut Vec<UiAction>
    ) {
        // Set beautiful dark theme
        self.setup_theme(ctx);

        // Beautiful header panel with gradient background
        egui::TopBottomPanel::top("header_panel")
            .exact_height(140.0)
            .show(ctx, |ui| {
                // Create beautiful background
                let rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 40, 80));

                ui.vertical_centered(|ui| {
                    ui.add_space(25.0);
                    
                    // Main title with beautiful styling
                    ui.heading(egui::RichText::new("PSD to PNG Converter")
                        .size(36.0)
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .strong());
                    
                    ui.add_space(8.0);
                    
                    // Subtitle
                    ui.label(egui::RichText::new("Transform your Adobe Photoshop files into beautiful PNG images")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(200, 220, 255)));
                    
                    ui.add_space(20.0);
                });
            });

        // Main content area - Clean professional interface
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                
                // Folder selection cards
                self.render_modern_folder_selection(ui, &input_folder, &output_folder, actions);
                
                ui.add_space(30.0);
                
                // Conversion controls
                self.render_modern_conversion_controls(ui, &input_folder, &output_folder, is_converting, &status_message, actions);
                
                ui.add_space(20.0);
                
                // Progress and status
                self.render_modern_progress_and_status(ui, is_converting, progress, status_message, error_message);
            });
        });

        // Beautiful footer
        egui::TopBottomPanel::bottom("footer_panel")
            .exact_height(50.0)
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(40, 50, 90));

                ui.vertical_centered(|ui| {
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("Made with Rust")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(180, 200, 255)));
                });
            });
    }

    fn render_drag_drop_area(&mut self, ui: &mut egui::Ui, actions: &mut Vec<UiAction>) {
        let drag_response = ui.add(
            egui::Button::new(
                egui::RichText::new("📁 Drop PSD folder here or click to browse")
                    .size(18.0)
            )
            .min_size(egui::vec2(400.0, 120.0))
            .fill(if self.drag_state.hovered {
                egui::Color32::from_rgb(60, 80, 120)
            } else {
                egui::Color32::from_gray(50)
            })
        );

        // Handle drag and drop
        if drag_response.hovered() {
            self.drag_state.hovered = true;
        } else {
            self.drag_state.hovered = false;
        }
        
        // Check for dropped files
        ui.ctx().input(|i| {
            for event in &i.raw.dropped_files {
                if let Some(path) = &event.path {
                    if path.is_dir() {
                        actions.push(UiAction::SetInputFolder(path.clone()));
                    }
                }
            }
        });

        // Handle click to browse
        if drag_response.clicked() {
            if let Some(folder) = FileDialog::new().pick_folder() {
                actions.push(UiAction::SetInputFolder(folder));
            }
        }

        // Visual feedback
        let rect = drag_response.rect;
        let stroke = if self.drag_state.hovered {
            egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 150, 255))
        } else {
            egui::Stroke::new(2.0, egui::Color32::from_gray(100))
        };
        
        ui.painter().rect_stroke(rect, 10.0, stroke);
    }

    fn render_folder_selection(&mut self, ui: &mut egui::Ui, input_folder: Option<PathBuf>, output_folder: Option<PathBuf>, actions: &mut Vec<UiAction>) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Input Folder:").size(16.0));
                
                let input_text = input_folder
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "No folder selected".to_string());
                
                ui.label(egui::RichText::new(input_text)
                    .color(egui::Color32::from_gray(200))
                    .monospace());
            });
            
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Output Folder:").size(16.0));
                
                let output_text = output_folder
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "No folder selected".to_string());
                
                ui.label(egui::RichText::new(output_text)
                    .color(egui::Color32::from_gray(200))
                    .monospace());
            });
        });

        ui.add_space(15.0);

        ui.horizontal_centered(|ui| {
            if ui.button(egui::RichText::new("📂 Select Input Folder").size(14.0))
                .clicked() {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    actions.push(UiAction::SetInputFolder(folder));
                }
            }
            
            ui.add_space(10.0);
            
            if ui.button(egui::RichText::new("📁 Select Output Folder").size(14.0))
                .clicked() {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    actions.push(UiAction::SetOutputFolder(folder));
                }
            }
        });
    }


    fn render_progress_bar(&mut self, ui: &mut egui::Ui, is_converting: bool, progress: f32) {
        if is_converting || progress > 0.0 {
            ui.add_space(20.0);
            
            let progress_bar = egui::ProgressBar::new(progress)
                .show_percentage();
            
            ui.centered_and_justified(|ui| {
                ui.add(progress_bar);
            });
            
            ui.add_space(10.0);
            
            ui.centered_and_justified(|ui| {
                ui.label(format!("{:.1}% Complete", progress * 100.0));
            });
        }
    }

    fn render_status_messages(&mut self, ui: &mut egui::Ui, is_converting: bool, status_message: String, error_message: Option<String>) {
        // Status message
        if !status_message.is_empty() {
            ui.centered_and_justified(|ui| {
                let color = if is_converting {
                    egui::Color32::from_rgb(100, 150, 255)
                } else if error_message.is_some() {
                    egui::Color32::from_rgb(255, 100, 100)
                } else {
                    egui::Color32::from_rgb(100, 255, 100)
                };
                
                ui.label(egui::RichText::new(&status_message)
                    .color(color)
                    .size(14.0));
            });
        }
        
        // Error message
        if let Some(error) = &error_message {
            ui.add_space(10.0);
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new(format!("❌ Error: {}", error))
                    .color(egui::Color32::from_rgb(255, 100, 100))
                    .size(14.0));
            });
        }
    }

    fn setup_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Beautiful dark theme colors
        style.visuals.window_fill = egui::Color32::from_rgb(25, 30, 50);
        style.visuals.panel_fill = egui::Color32::from_rgb(30, 35, 60);
        style.visuals.faint_bg_color = egui::Color32::from_rgb(40, 45, 70);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(20, 25, 45);
        
        // Button styling
        style.visuals.button_frame = true;
        style.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 70, 100));
        
        // Note: Text colors are handled by RichText styling
        
        ctx.set_style(style);
    }

    fn render_modern_drag_drop_area(&mut self, ui: &mut egui::Ui, actions: &mut Vec<UiAction>) {
        ui.vertical_centered(|ui| {
            let drag_response = ui.add(
                egui::Button::new(
                    egui::RichText::new("📁 Drop PSD folder here or click to browse")
                        .size(18.0)
                        .color(egui::Color32::from_rgb(255, 255, 255))
                )
                .min_size(egui::vec2(500.0, 120.0))
                .fill(if self.drag_state.hovered {
                    egui::Color32::from_rgb(60, 80, 140)
                } else {
                    egui::Color32::from_rgb(40, 55, 100)
                })
            );

            // Handle hover state
            if drag_response.hovered() {
                self.drag_state.hovered = true;
            } else {
                self.drag_state.hovered = false;
            }
            
            // Check for dropped files
            ui.ctx().input(|i| {
                for event in &i.raw.dropped_files {
                    if let Some(path) = &event.path {
                        if path.is_dir() {
                            actions.push(UiAction::SetInputFolder(path.clone()));
                        }
                    }
                }
            });

            // Handle click to browse
            if drag_response.clicked() {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    actions.push(UiAction::SetInputFolder(folder));
                }
            }

            // Beautiful border with rounded corners
            let rect = drag_response.rect;
            let stroke_color = if self.drag_state.hovered {
                egui::Color32::from_rgb(100, 150, 255)
            } else {
                egui::Color32::from_rgb(70, 90, 150)
            };
            
            ui.painter().rect_stroke(rect, 15.0, egui::Stroke::new(3.0, stroke_color));
        });
    }

    fn render_modern_folder_selection(&mut self, ui: &mut egui::Ui, input_folder: &Option<PathBuf>, output_folder: &Option<PathBuf>, actions: &mut Vec<UiAction>) {
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("📂 Folder Selection")
                .size(20.0)
                .color(egui::Color32::from_rgb(220, 230, 255))
                .strong());
            
            ui.add_space(20.0);
            
            // Create two beautiful cards for input and output - truly centered
            ui.horizontal(|ui| {
                // Calculate total width needed for both cards plus gap
                let card_width = 280.0;
                let gap_width = 20.0;
                let total_width = (card_width * 2.0) + gap_width;
                let available_width = ui.available_width();
                let left_space = (available_width - total_width) / 2.0;
                
                // Add equal spacing on left and right
                ui.add_space(left_space);
                
                // Input folder card
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(280.0, 120.0));
                    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("📂 Input Folder")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(100, 200, 255)));
                        });
                        
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            let input_text = input_folder
                                .as_ref()
                                .map(|p| {
                                    let path_str = p.to_string_lossy().to_string();
                                    if path_str.len() > 35 {
                                        format!("...{}", &path_str[path_str.len()-32..])
                                    } else {
                                        path_str
                                    }
                                })
                                .unwrap_or_else(|| "No folder selected".to_string());
                            
                            ui.label(egui::RichText::new(input_text)
                                .color(egui::Color32::from_rgb(180, 190, 220))
                                .monospace()
                                .size(12.0));
                        });
                        
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            if ui.add(egui::Button::new(egui::RichText::new("📁 Browse")
                                .size(14.0)
                                .color(egui::Color32::from_rgb(255, 255, 255)))
                                .fill(egui::Color32::from_rgb(60, 120, 200)))
                                .clicked() {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    actions.push(UiAction::SetInputFolder(folder));
                                }
                            }
                        });
                        ui.add_space(15.0);
                    });
                });
                
                ui.add_space(20.0);
                
                // Output folder card
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(280.0, 120.0));
                    
                    ui.vertical(|ui| {
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("📁 Output Folder")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(255, 200, 100)));
                        });
                        
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            let output_text = output_folder
                                .as_ref()
                                .map(|p| {
                                    let path_str = p.to_string_lossy().to_string();
                                    if path_str.len() > 35 {
                                        format!("...{}", &path_str[path_str.len()-32..])
                                    } else {
                                        path_str
                                    }
                                })
                                .unwrap_or_else(|| "No folder selected".to_string());
                            
                            ui.label(egui::RichText::new(output_text)
                                .color(egui::Color32::from_rgb(180, 190, 220))
                                .monospace()
                                .size(12.0));
                        });
                        
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.add_space(15.0);
                            if ui.add(egui::Button::new(egui::RichText::new("📁 Browse")
                                .size(14.0)
                                .color(egui::Color32::from_rgb(255, 255, 255)))
                                .fill(egui::Color32::from_rgb(200, 140, 60)))
                                .clicked() {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    actions.push(UiAction::SetOutputFolder(folder));
                                }
                            }
                        });
                        ui.add_space(15.0);
                    });
                });
                
                // Add equal spacing on the right
                ui.add_space(left_space);
            });
        });
    }

    fn render_modern_conversion_controls(&mut self, ui: &mut egui::Ui, input_folder: &Option<PathBuf>, output_folder: &Option<PathBuf>, is_converting: bool, status_message: &str, actions: &mut Vec<UiAction>) {
        ui.vertical_centered(|ui| {
            let can_convert = input_folder.is_some() && output_folder.is_some() && !is_converting;
            let has_completed = status_message.contains("completed");
            
            if is_converting {
                // Converting state
                ui.add(
                    egui::Button::new(
                        egui::RichText::new("⏳ Converting...")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(255, 255, 255))
                    )
                    .min_size(egui::vec2(400.0, 80.0))
                    .fill(egui::Color32::from_rgb(100, 150, 200))
                );
            } else if has_completed {
                // Finished state - disabled
                ui.add(
                    egui::Button::new(
                        egui::RichText::new("✅ CONVERSION FINISHED - SELECT NEW FILES")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(100, 100, 100))
                    )
                    .min_size(egui::vec2(400.0, 80.0))
                    .fill(egui::Color32::from_rgb(60, 60, 60))
                );
            } else if can_convert {
                // Ready to convert state - only when BOTH folders are selected
                let button_response = ui.add(
                    egui::Button::new(
                        egui::RichText::new("🚀 START CONVERSION")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(255, 255, 255))
                            .strong()
                    )
                    .min_size(egui::vec2(400.0, 80.0))
                    .fill(egui::Color32::from_rgb(0, 200, 100))  // Professional green
                );
                
                if button_response.clicked() {
                    actions.push(UiAction::StartConversion);
                }
            } else {
                // Not ready state - show specific guidance
                let missing_folders = match (input_folder.is_some(), output_folder.is_some()) {
                    (false, false) => "Select input and output folders",
                    (false, true) => "Select input folder",
                    (true, false) => "Select output folder",
                    (true, true) => "Ready to convert", // This shouldn't happen due to can_convert logic
                };
                
                ui.label(egui::RichText::new(missing_folders)
                    .size(18.0)
                    .color(egui::Color32::from_rgb(255, 180, 0)));
            }
        });
    }

    fn render_modern_progress_and_status(&mut self, ui: &mut egui::Ui, is_converting: bool, progress: f32, status_message: String, error_message: Option<String>) {
        ui.vertical_centered(|ui| {
            // Progress bar
            if is_converting || progress > 0.0 {
                ui.add_space(20.0);
                
                let progress_bar = egui::ProgressBar::new(progress)
                    .show_percentage()
                    .fill(egui::Color32::from_rgb(100, 200, 255));
                
                ui.add(progress_bar);
                
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new(format!("{:.1}% Complete", progress * 100.0))
                    .size(16.0)
                    .color(egui::Color32::from_rgb(150, 200, 255)));
            }
            
            // Status message
            if !status_message.is_empty() {
                ui.add_space(15.0);
                let color = if is_converting {
                    egui::Color32::from_rgb(100, 200, 255)
                } else if error_message.is_some() {
                    egui::Color32::from_rgb(255, 120, 120)
                } else {
                    egui::Color32::from_rgb(120, 255, 120)
                };
                
                ui.label(egui::RichText::new(&status_message)
                    .color(color)
                    .size(16.0));
            }
            
            // Error message
            if let Some(error) = &error_message {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("❌ Error: {}", error))
                    .color(egui::Color32::from_rgb(255, 120, 120))
                    .size(16.0));
            }
        });
    }
}
