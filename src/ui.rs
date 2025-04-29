use eframe::egui;
use rfd::FileDialog; // Импортируем FileDialog
use std::fs;

pub struct MyIde {
    code: String,
    output: String,
    file_path: Option<String>, // Путь к открытому файлу
    is_dark_mode: bool, // Добавляем поле для переключения темы
    transition_progress: f32, // Прогресс перехода (от 0 до 1)
}

impl Default for MyIde {
    fn default() -> Self {
        Self {
            code: String::new(),
            output: String::from("Здесь будет вывод."),
            file_path: None, // Изначально путь пустой
            is_dark_mode: true, // Начинаем с темной темы
            transition_progress: 0.0, // Начальный прогресс перехода
        }
    }
}

impl eframe::App for MyIde {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Переключение темы с плавным переходом
        if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
            self.is_dark_mode = !self.is_dark_mode;
            self.transition_progress = 0.0; // Начинаем плавный переход
            ctx.request_repaint(); // Запрашиваем повторную отрисовку
        }

        // Обработчик плавного перехода
        if self.transition_progress < 1.0 {
            self.transition_progress += 0.05; // Увеличиваем прогресс перехода
            if self.transition_progress > 1.0 {
                self.transition_progress = 1.0;
            }

            // Применяем плавный эффект изменения визуалов
            let visuals = if self.is_dark_mode {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };
            ctx.set_visuals(egui::Visuals {
                dark_mode: self.is_dark_mode,
                ..visuals
            });
        }

        // Обрабатываем нажатие клавиш (например, CTRL+S) для сохранения файла
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save_file();
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
                    self.output = "Запуск (заглушка)...".to_string();
                }

                if ui.button("💾 Сохранить").clicked() {
                    self.save_file();
                }

                // Кнопка для изменения темы
                if ui.button(if self.is_dark_mode { "🌞 Светлая тема" } else { "🌜 Тёмная тема" }).clicked() {
                    self.is_dark_mode = !self.is_dark_mode;
                    self.transition_progress = 0.0; // Начинаем плавный переход
                    ctx.request_repaint(); // Запрашиваем повторную отрисовку
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

impl MyIde {
    fn save_file(&mut self) {
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
}
