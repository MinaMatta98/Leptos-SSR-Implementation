use gloo_net::http::Request;
use leptos::html::*;
use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsObject, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Event, FileReader, HtmlDivElement, HtmlFormElement, HtmlIFrameElement, HtmlImageElement,
    HtmlInputElement, MouseEvent, SubmitEvent,
};

pub struct CallbackClosure;

impl CallbackClosure {
    pub fn remove_callback(
        node_ref: NodeRef<Iframe>,
        pdf_iframe_setter: WriteSignal<String>,
        toggle: WriteSignal<bool>,
    ) -> impl Fn(MouseEvent) {
        move |_: MouseEvent| {
            let document = document().unchecked_into::<web_sys::HtmlDocument>();
            let iframe = node_ref.get().expect("iframe could not be captured");

            let src = iframe
                .src()
                .split('/')
                .last()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();

            spawn_local(async move {
                remove_file(src.clone()).await.unwrap();
                match randomizer().await.unwrap() {
                    Some(pdf_iri) => {
                        pdf_iframe_setter.set(format!("/pdf/{pdf_iri}"));

                        document
                            .get_element_by_id(&format!("/thumbnails/{src}.png"))
                            .unwrap()
                            .dyn_into::<HtmlDivElement>()
                            .unwrap()
                            .remove();
                    }
                    None => toggle.update(|value: &mut bool| *value = !*value),
                }
            })
        }
    }
    pub fn submit_callback<'a>(
        file_input_ref: NodeRef<Input>,
        response_setter: WriteSignal<leptos::Fragment>,
        cx: Scope,
        thumbnail_refresh_setter: Option<WriteSignal<bool>>,
        submit_button_setter: Option<WriteSignal<&'a str>>,
        upload_label_setter: Option<WriteSignal<&'a str>>,
        toggle: Option<WriteSignal<bool>>
    ) -> impl Fn(SubmitEvent) {
        move |event: SubmitEvent| {
            event.stop_propagation();
            event.prevent_default();

            let file_input = file_input_ref.get().expect("could not capture file input");

            let file = file_input.files().unwrap().get(0).unwrap();

            let buffer = file_to_u8(file);

            spawn_local(async move {
                let buffer = buffer.await.unwrap();

                let result = Request::post("/submit").body(&buffer).send().await.unwrap();

                if result.ok() {
                    response_setter.set(view! {cx,
                          <p class="mx-1">"File Uploaded Successfully "</p>
                          <i class="fas fa-check-circle text-success" style="font-size: 1em"></i>
                });
                    if let Some(refresher) = thumbnail_refresh_setter {
                        refresher.update(|value: &mut bool| *value = !*value);
                    }

                    if let Some(toggle) = toggle {
                        toggle.update(|val: &mut bool| *val = !*val);
                    }

                } else {
                    let response_text = result.text().await.unwrap();

                    response_setter.set(view! {cx,
                          <p class="mx-1">{response_text}</p>
                          <i class="fas fa-circle-xmark text-danger" style="font-size: 1em"></i>
                    });
                }

                if let Some(setter) = submit_button_setter {
                    setter.set("display:none");
                }

                if let Some(setter) = upload_label_setter {
                    setter.set("uploadLabel display-box");
                }
            })
        }
    }

    pub fn submit_handler<'a>(
        file_input_ref: NodeRef<Input>,
        submit_button_setter: WriteSignal<&'a str>,
        submit_button_class_setter: WriteSignal<String>,
        upload_label_setter: WriteSignal<&'a str>,
    ) -> impl Fn(Event) {
        move |event: web_sys::Event| {
            let target = file_input_ref.get().unwrap();

            if let Some(files) = target.files() {
                if files.length().gt(&0) {
                    submit_button_setter.set("display: block");
                    submit_button_class_setter
                        .set(String::from("submit-button submit-button-large mb-3"));
                    upload_label_setter.set("uploadLabel display-box uploadLabel-small");
                }
            }
        }
    }
}

#[server(RemoveFile)]
async fn remove_file(path: String) -> Result<(), ServerFnError> {
    let files = [
        format!("./upload/{path}.pdf"),
        format!("./upload/thumbnails/{path}.png"),
    ];
    files
        .iter()
        .for_each(|file| std::fs::remove_file(file).unwrap_or_default());
    Ok(())
}

#[server(Random)]
pub async fn randomizer() -> Result<Option<String>, ServerFnError> {
    use rand::prelude::*;
    let entries = std::fs::read_dir("./upload").unwrap();

    let mut file_paths: Vec<std::path::PathBuf> = entries
        .map(|entry| {
            entry
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))
                .unwrap()
                .path()
        })
        .filter(|path| path.is_file())
        .collect();

    let mut rng = rand::thread_rng();
    file_paths.shuffle(&mut rng);
    let rand_file = match file_paths.first() {
        Some(val) => val,
        None => return Ok(None),
    };

    let parent = match rand_file.parent() {
        Some(val) => val.display().to_string(),
        None => return Ok(None),
    };

    match rand_file
        .display()
        .to_string()
        .replace(&(parent + "/"), "")
        .split('.')
        .next()
    {
        Some(val) => Ok(Some(val.to_string())),
        None => Ok(None),
    }
}

#[wasm_bindgen]
pub async fn file_to_u8(file: web_sys::File) -> Result<js_sys::Uint8Array, JsValue> {
    let buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
        .await
        .unwrap();
    let u8_array = js_sys::Uint8Array::new(&buffer);
    Ok(u8_array)
}
