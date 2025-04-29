mod ui; // Импортируем модуль ui
use ui::MyIde; // Импортируем структуру MyIde

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Rust IDE", options, Box::new(|_cc| Box::new(MyIde::default())))
}
