use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys;
use log::{info, warn};

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_file_picker(file_content: Rc<RefCell<String>>) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{Event, HtmlInputElement};

    info!("File picker opened");

    // Создаем элемент <input type="file">
    let document = web_sys::window().unwrap().document().unwrap();
    let input = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.set_type("file");
    input.set_accept("application/json"); // Принимаем только текстовые файлы
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
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Event, FileReader};

    info!("Reading file...");

    let reader = FileReader::new().unwrap();

    let reader_c = reader.clone();
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onload = Closure::once(Box::new(move |_event: Event| {
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut(_)>);

        let onerror = Closure::once(Box::new(move |_event: Event| {
            reject.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut(_)>);

        reader_c.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        reader_c.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    });

    reader.read_as_text(&file).unwrap();

    // Ожидаем завершения чтения файла
    JsFuture::from(promise).await.unwrap();

    let result = reader.result().unwrap();
    let text = result.as_string().unwrap();

    // Сохраняем содержимое файла в состоянии приложения
    *file_content.borrow_mut() = text;
}