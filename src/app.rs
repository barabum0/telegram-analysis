use std::cell::RefCell;
use std::rc::Rc;

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
        egui::CentralPanel::default().show(ctx, |ui| {
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

#[cfg(target_arch = "wasm32")]
fn open_file_picker(file_content: Rc<RefCell<String>>) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{Event, HtmlInputElement};

    // Создаем элемент <input type="file">
    let document = web_sys::window().unwrap().document().unwrap();
    let input = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.set_type("file");
    input.set_accept("text/*"); // Принимаем только текстовые файлы
    input.set_multiple(false);   // Разрешить выбор нескольких файлов, если нужно

    // Обработчик события изменения (когда пользователь выбирает файл)
    let file_content_clone = file_content.clone();
    let closure = Closure::wrap(Box::new(move |event: Event| {
        let input = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if let Some(files) = input.files() {
            if files.length() > 0 {
                let file = files.get(0).unwrap();
                wasm_bindgen_futures::spawn_local(read_file(file, file_content_clone.clone()));
            }
        }
    }) as Box<dyn FnMut(_)>);

    input
        .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Предотвращаем сборку мусора для closure

    // Инициируем событие клика, чтобы открыть диалог выбора файла
    input.click();
}

#[cfg(target_arch = "wasm32")]
async fn read_file(file: web_sys::File, file_content: Rc<RefCell<String>>) {
    use futures::channel::oneshot;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{FileReader, ProgressEvent};

    let reader = FileReader::new().unwrap();

    let (sender, receiver) = oneshot::channel();

    let onload = Closure::once(Box::new(move |_event: ProgressEvent| {
        let _ = sender.send(());
    }) as Box<dyn FnMut(_)>);

    reader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    reader.read_as_text(&file).unwrap();

    // Ожидаем завершения чтения файла
    receiver.await.unwrap();

    let result = reader.result().unwrap();
    let text = result.as_string().unwrap();

    // Сохраняем содержимое файла в состоянии приложения
    *file_content.borrow_mut() = text;
}