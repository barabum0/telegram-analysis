use std::cell::RefCell;
use std::rc::Rc;
use egui_notify::Toasts;
use log::info;

#[cfg(target_arch = "wasm32")]
use crate::file_reader::open_file_picker;

pub(crate) struct MyApp {
    file_content: Rc<RefCell<String>>,
    toasts: Toasts,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_content: Rc::new(RefCell::new("".to_string())),
            toasts: Toasts::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.toasts.show(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let file_content = self.file_content.clone();

            ui.heading("Привет, egui на вебе!");

            if ui.button("Открыть файл").clicked() {
                #[cfg(target_arch = "wasm32")]
                {
                    self.toasts.info("Открываю файл");
                    info!("Вызываю файл");
                    open_file_picker(file_content.clone());
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.toasts.error("Открытие файлов доступно только в веб-версии.");
                }
            }

            ui.separator();

            ui.label("Содержимое файла:");
            ui.code(&*file_content.borrow());
        });
    }
}