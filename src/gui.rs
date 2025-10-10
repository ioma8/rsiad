use eframe::egui;
use rsiad::{
    VocalExerciseEngine, ExerciseConfig, OutputMode, 
    ExerciseType, ToneRange, stop_playback,
};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "RSIAD - Vocal Warmup Exercise",
        options,
        Box::new(|cc| {
            // Use Catppuccin Mocha theme (dark)
            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
            Box::new(VocalApp::default())
        }),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlaybackState {
    Idle,
    Playing,
    Generating,
}

struct VocalApp {
    // Configuration
    soundfont_path: String,
    exercise_type: ExerciseType,
    vocal_range: Option<ToneRange>,
    custom_from: String,
    custom_to: String,
    use_custom_range: bool,
    note_duration: f32,
    save_path: String,
    
    // State
    playback_state: Arc<Mutex<PlaybackState>>,
    status_message: Arc<Mutex<String>>,
    error_message: Arc<Mutex<String>>,
}

impl Default for VocalApp {
    fn default() -> Self {
        Self {
            soundfont_path: "UprightPianoKW-small-bright-20190703.sf2".to_string(),
            exercise_type: ExerciseType::Triads,
            vocal_range: Some(ToneRange::Baritone),
            custom_from: "A2".to_string(),
            custom_to: "A4".to_string(),
            use_custom_range: false,
            note_duration: 0.8,
            save_path: "output.mp3".to_string(),
            playback_state: Arc::new(Mutex::new(PlaybackState::Idle)),
            status_message: Arc::new(Mutex::new("Ready".to_string())),
            error_message: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl VocalApp {
    fn get_key_range(&self) -> (u8, u8) {
        if self.use_custom_range {
            let from = VocalExerciseEngine::note_string_to_key(&self.custom_from);
            let to = VocalExerciseEngine::note_string_to_key(&self.custom_to);
            (from, to)
        } else if let Some(range) = self.vocal_range {
            VocalExerciseEngine::get_vocal_range_keys(range)
        } else {
            (57, 81) // Default baritone
        }
    }
    
    fn play_realtime(&mut self) {
        let engine = VocalExerciseEngine::new(&self.soundfont_path);
        let (key_from, key_to) = self.get_key_range();
        
        let config = ExerciseConfig {
            exercise_type: self.exercise_type,
            key_range: (key_from, key_to),
            note_duration: self.note_duration as f64,
            vocal_range: self.vocal_range,
        };
        
        let state = Arc::clone(&self.playback_state);
        let status = Arc::clone(&self.status_message);
        let error = Arc::clone(&self.error_message);
        
        *state.lock().unwrap() = PlaybackState::Playing;
        *status.lock().unwrap() = "Playing...".to_string();
        error.lock().unwrap().clear();
        
        thread::spawn(move || {
            match engine.generate_exercise(config, OutputMode::Realtime) {
                Ok(result) => {
                    *status.lock().unwrap() = format!("Completed ({} notes)", result.notes_played);
                }
                Err(e) => {
                    *error.lock().unwrap() = format!("Error: {}", e);
                    *status.lock().unwrap() = "Error".to_string();
                }
            }
            *state.lock().unwrap() = PlaybackState::Idle;
        });
    }
    
    fn save_to_file(&mut self) {
        let engine = VocalExerciseEngine::new(&self.soundfont_path);
        let (key_from, key_to) = self.get_key_range();
        
        let config = ExerciseConfig {
            exercise_type: self.exercise_type,
            key_range: (key_from, key_to),
            note_duration: self.note_duration as f64,
            vocal_range: self.vocal_range,
        };
        
        let output_path = self.save_path.clone();
        let state = Arc::clone(&self.playback_state);
        let status = Arc::clone(&self.status_message);
        let error = Arc::clone(&self.error_message);
        
        *state.lock().unwrap() = PlaybackState::Generating;
        *status.lock().unwrap() = "Generating MP3...".to_string();
        error.lock().unwrap().clear();
        
        thread::spawn(move || {
            match engine.generate_exercise(config, OutputMode::File { path: output_path.clone() }) {
                Ok(_result) => {
                    *status.lock().unwrap() = format!("Saved: {}", output_path);
                }
                Err(e) => {
                    *error.lock().unwrap() = format!("Error: {}", e);
                    *status.lock().unwrap() = "Error".to_string();
                }
            }
            *state.lock().unwrap() = PlaybackState::Idle;
        });
    }
    
    fn stop(&mut self) {
        stop_playback();
        *self.playback_state.lock().unwrap() = PlaybackState::Idle;
        *self.status_message.lock().unwrap() = "Stopped".to_string();
    }
}

impl eframe::App for VocalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_state = *self.playback_state.lock().unwrap();
        let status_message = self.status_message.lock().unwrap().clone();
        let error_message = self.error_message.lock().unwrap().clone();
        
        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("🎵 RSIAD").size(22.0).strong());
                ui.label(egui::RichText::new("Vocal Warmup Exercise").size(11.0));
            });
            
            ui.add_space(16.0);
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Soundfont Section
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🎹 Soundfont").strong());
                    ui.add_space(6.0);
                    
                    ui.horizontal(|ui| {
                        let text_width = ui.available_width() - 80.0;
                        ui.add(egui::TextEdit::singleline(&mut self.soundfont_path)
                            .desired_width(text_width));
                        if ui.button("Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("SoundFont", &["sf2", "sf3"])
                                .pick_file()
                            {
                                self.soundfont_path = path.display().to_string();
                            }
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Exercise Settings
                ui.group(|ui| {
                    ui.label(egui::RichText::new("⚙️ Exercise Settings").strong());
                    ui.add_space(6.0);
                    
                    // Exercise Type
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        ui.radio_value(&mut self.exercise_type, ExerciseType::Triads, "Triads");
                        ui.radio_value(&mut self.exercise_type, ExerciseType::Scales, "Scales");
                        ui.radio_value(&mut self.exercise_type, ExerciseType::Octaves, "Octaves");
                    });
                    
                    ui.add_space(8.0);
                    
                    // Vocal Range
                    ui.horizontal(|ui| {
                        ui.label("Range:");
                        if !self.use_custom_range {
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", self.vocal_range.unwrap_or(ToneRange::Baritone)))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::Bass), "Bass (E2-E4)");
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::Baritone), "Baritone (A2-A4)");
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::Tenor), "Tenor (C3-C5)");
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::Alto), "Alto (F3-F5)");
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::MezzoSoprano), "Mezzo-Soprano (A3-A5)");
                                    ui.selectable_value(&mut self.vocal_range, Some(ToneRange::Soprano), "Soprano (C4-C6)");
                                });
                        }
                        ui.checkbox(&mut self.use_custom_range, "Custom");
                    });
                    
                    if self.use_custom_range {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("From:");
                            ui.add(egui::TextEdit::singleline(&mut self.custom_from).desired_width(55.0));
                            ui.add_space(6.0);
                            ui.label("To:");
                            ui.add(egui::TextEdit::singleline(&mut self.custom_to).desired_width(55.0));
                        });
                    }
                    
                    ui.add_space(8.0);
                    
                    // Duration
                    ui.horizontal(|ui| {
                        ui.label("Duration:");
                        ui.add(egui::Slider::new(&mut self.note_duration, 0.1..=2.0)
                            .text("s")
                            .custom_formatter(|n, _| format!("{:.1}s", n)));
                    });
                });
                
                ui.add_space(10.0);
                
                // Output Section
                ui.group(|ui| {
                    ui.label(egui::RichText::new("💾 Output").strong());
                    ui.add_space(6.0);
                    
                    ui.horizontal(|ui| {
                        let text_width = ui.available_width() - 80.0;
                        ui.add(egui::TextEdit::singleline(&mut self.save_path)
                            .desired_width(text_width));
                        if ui.button("Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("MP3", &["mp3"])
                                .set_file_name(&self.save_path)
                                .save_file()
                            {
                                self.save_path = path.display().to_string();
                            }
                        }
                    });
                });
                
                ui.add_space(14.0);
                
                // Action Buttons
                let is_idle = current_state == PlaybackState::Idle;
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    
                    let button_width = (ui.available_width() - 16.0) / 3.0;
                    
                    if ui.add_enabled(is_idle, 
                        egui::Button::new("▶ Play")
                            .min_size(egui::vec2(button_width, 36.0))
                    ).clicked() {
                        self.play_realtime();
                    }
                    
                    if ui.add_enabled(is_idle, 
                        egui::Button::new("💾 Save MP3")
                            .min_size(egui::vec2(button_width, 36.0))
                    ).clicked() {
                        self.save_to_file();
                    }
                    
                    if ui.add_enabled(!is_idle, 
                        egui::Button::new("⏹ Stop")
                            .min_size(egui::vec2(button_width, 36.0))
                    ).clicked() {
                        self.stop();
                    }
                });
                
                ui.add_space(14.0);
                
                // Status Section
                ui.group(|ui| {
                    let status_icon = match current_state {
                        PlaybackState::Playing => "▶",
                        PlaybackState::Generating => "⚙",
                        PlaybackState::Idle => "●",
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(status_icon).size(14.0));
                        ui.label(egui::RichText::new("Status:").strong());
                        ui.label(&status_message);
                    });
                    
                    if !error_message.is_empty() {
                        ui.add_space(4.0);
                        ui.colored_label(egui::Color32::from_rgb(240, 100, 100), 
                            format!("⚠ {}", error_message));
                    }
                    
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);
                    
                    let (from, to) = self.get_key_range();
                    ui.label(egui::RichText::new(
                        format!("Range: {} → {} (MIDI {}-{})", self.custom_from, self.custom_to, from, to)
                    ).size(11.0));
                    
                    let range_size = match self.exercise_type {
                        ExerciseType::Triads | ExerciseType::Scales => {
                            if to > from { (to - from).saturating_sub(7) + 1 } else { 0 }
                        }
                        ExerciseType::Octaves => {
                            if to > from { (to - from).saturating_sub(12) + 1 } else { 0 }
                        }
                    };
                    
                    if range_size > 0 {
                        let estimated_duration = range_size as f32 * self.note_duration * 6.0;
                        ui.label(egui::RichText::new(
                            format!("Estimated: {:.1}s • {} keys", estimated_duration, range_size)
                        ).size(11.0));
                    } else {
                        ui.colored_label(egui::Color32::from_rgb(255, 180, 100), 
                            "⚠ Invalid range for exercise type");
                    }
                });
                
                ui.add_space(8.0);
            });
        });
    }
}
