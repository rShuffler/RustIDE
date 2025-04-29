use eframe::egui;
use rfd::FileDialog; // Импортируем FileDialog
use std::fs; 

pub struct MyIde {
    code: String,
    output: String,
    file_path: Option<String>, // Путь к открытому файлу
}

impl Default for MyIde {
    fn default() -> Self {
        Self {
            code: String::new(),
            output: String::from("Здесь будет вывод."),
            file_path: None, // Изначально путь пустой
        }
    }
}

impl eframe::App for MyIde {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Обрабатываем нажатие клавиш (например, CTRL+S)
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            if let Some(path) = &self.file_path {
                if let Err(e) = fs::write(path, &self.code) {
                    self.output = format!("Ошибка сохранения: {}", e);
                } else {
                    self.output = format!("Файл сохранён: {}", path);
                }
            } else {
                self.output = "Нет файла для сохранения.".to_string();
            }
        }

        // Верхняя панель с кнопками
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🔷 Rust IDE");
                ui.separator();
                if ui.button("Открыть файл").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            self.code = contents;
                            self.file_path = Some(path.to_string_lossy().to_string()); // Сохраняем путь
                        }
                    }
                }
                if ui.button("▶ Запустить").clicked() {
                    self.output = "Запуск недоступен...".to_string();
                }
            });
        });

        // Боковая панель (слева)
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(150.0)  // Ширина панели
            .show(ctx, |ui| {
                ui.heading("Файлы");
                ui.separator();
                ui.label("📄 main.rs");
                ui.label("📄 test.rs");
            });

        // Центральная панель с редактором и терминалом
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Код:");
                ui.add_sized([ui.available_width(), 300.0], egui::TextEdit::multiline(&mut self.code));
                ui.separator();
                ui.label("Терминал:");
                ui.add_sized([ui.available_width(), 150.0], egui::TextEdit::multiline(&mut self.output));
            });
        });
    }
}
