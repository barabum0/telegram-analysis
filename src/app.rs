use std::cell::RefCell;
use std::rc::Rc;
use crate::file_reader::open_file_picker;

pub(crate) struct MyApp {
    file_content: Rc<RefCell<String>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_content: Rc::new(RefCell::new("".to_string())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            let file_content = self.file_content.clone();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Привет, egui на вебе!");

                if ui.button("Открыть файл").clicked() {
                    #[cfg(target_arch = "wasm32")]
                    {
                        open_file_picker(file_content.clone());
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        ui.label("Открытие файлов доступно только в веб-версии.");
                    }
                }

                ui.separator();

                ui.label("Содержимое файла:");
                ui.code(&*file_content.borrow());
            });
        });
    }
}