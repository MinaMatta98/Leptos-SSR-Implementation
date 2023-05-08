use crate::styles::augment::*;
use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, MouseEvent};
pub mod callback;
use callback::CallbackClosure;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UploadForm {
    pub file: Vec<u8>,
}

#[component]
pub fn Augment(cx: Scope) -> impl IntoView {
    let (signal, signal_setter) = create_signal(cx, false);
    let res = create_resource(cx, signal, |_| async move { server_func().await });

    view! {cx,
        <Router>
        <UploadStyles/>
        <Suspense fallback=move || view! {cx, <div></div>}>
        { move || res.read(cx).map(|val| view! {cx,
         <Show
            when=move|| {
                val.clone().unwrap()
           }
            fallback = move |cx| view! { cx, <DragDrop toggle = signal_setter/> }> <FilePresent toggle = signal_setter />
         </Show>
        })}
        </Suspense>
        </Router>
    }
}

#[component]
fn DragDrop(cx: Scope, #[prop(into)] toggle: WriteSignal<bool>) -> impl IntoView {
    let (submit_style, submit_style_setter) = create_signal(cx, "display:none");
    let (upload_label, upload_label_setter) = create_signal(cx, "uploadLabel display-box");
    let (submit_class, submit_class_setter) = create_signal(cx, "");
    let (response, response_setter) = create_signal(cx, view! {cx, <p/><i/>});
    let file_input_ref = create_node_ref::<leptos::html::Input>(cx);

    let submit_button_handler = move |_event: web_sys::Event| {
        let target = file_input_ref.get().unwrap();
        if let Some(files) = target.files() {
            if files.length().gt(&0) {
                submit_class_setter.set("submit-button submit-button-large");
            }
        }
        submit_style_setter.set("display: unset");
        upload_label_setter.set("uploadLabel display-box uploadLabel-small");
    };

    let on_submit_callback = CallbackClosure::submit_callback(
        file_input_ref,
        response_setter,
        cx,
        None,
        Some(submit_class_setter),
        Some(upload_label_setter),
        Some(toggle),
    );
    let style = drag_styles();

    styled::view! {cx, style,
    <Router>
    <br />
    <br />
    <br />
    <br />
    <br />
    <br />
    <h1 class="text-center position-absolute translate-middle-x start-50 top-25">"Advanced Search Made Simple"</h1>
    <br />
    <section id="drag-drop-section">
      <div id="drop-area" class="my-5">
        <div class="progress-container">
          <div id="progress-bar" class="progress-bar">
            <span class="progress-value"></span>
          </div>
        </div>
            {response}
        <i class="fa-solid fa-file-import"></i>
        <div id="upload-instructions">
          <Form method="post" action="/submit" enctype=String::from("multipart/form-data") on:submit=on_submit_callback>
            <h2>"Drag and drop files here"</h2>
            <h2>"OR"</h2>
            <div>
                <label class={upload_label} for="file-input" id="uploadLabel">"Choose File"</label>
            <input id="file-input" _ref=file_input_ref type="file" class="upload" name="file-input" accept="application/pdf" on:change=submit_button_handler />
            </div>
            <br />
            <div>
                <input style=submit_style class=submit_class type="submit" id="submit-button" value="Submit">"Submit"</input>
            </div>
          </Form>
        </div>
      </div>
    </section>
        </Router>
    }
}

#[component]
fn FilePresent(cx: Scope, #[prop(into)] toggle: WriteSignal<bool>) -> impl IntoView {
    let (submit_button, submit_button_setter) = create_signal(cx, "display: none");
    let (submit_button_class, submit_button_class_setter) = create_signal(cx, "mb-3".to_string());
    let (upload_label, upload_label_setter) = create_signal(cx, "uploadLabel display-box");
    let (pdf_iframe, pdf_iframe_setter) = create_signal(cx, "".to_string());
    let (response, response_setter) = create_signal(cx, view! {cx, <p/><i/>});
    let (thumbnail_refresh, thumbnail_refresh_setter) = create_signal(cx, false);
    let iframe_ref = create_node_ref::<leptos::html::Iframe>(cx);
    let file_input_ref = create_node_ref::<leptos::html::Input>(cx);
    let pdf_path = create_server_action::<callback::Random>(cx);
    pdf_path.dispatch(callback::Random {});

    let remove_callback = CallbackClosure::remove_callback(iframe_ref, pdf_iframe_setter, toggle);
    let submit_button_handler = CallbackClosure::submit_handler(
        file_input_ref,
        submit_button_setter,
        submit_button_class_setter,
        upload_label_setter,
    );
    let on_submit_callback = CallbackClosure::submit_callback(
        file_input_ref,
        response_setter,
        cx,
        Some(thumbnail_refresh_setter),
        Some(submit_button_setter),
        Some(upload_label_setter),
        None,
    );

    styled::view! {cx, drag_styles(),
    <Router>
    <ThumbnailList pdf_iframe_setter thumbnail_refresh/>
    <h1 class="position-absolute w-75 start-50 translate-middle-x text-center p-5 mx-5">"Advanced Search Made Simple"</h1>
    <div id="status" style="display: none;"
        class="position-absolute w-75 start-50 translate-middle-x d-flex h-25 mx-5 my-3">
    </div>
    <div class="loading position-absolute start-50 bottom-50 my-5 translate-center-x mx-4" style="display: none;"
        id="loading-animation"></div>
     <div style="display: block ruby" class="position-absolute w-75 start-50 translate-middle-x bottom-50 h-25 mx-5 text-center my-5">
        {response}
     </div>
    <form method="post" action="/submit" enctype="multipart/form-data"
        style="justify-content: center; display: flex;  flex-direction: column"
        class="position-absolute w-75 start-50 translate-middle-x bottom-50 h-25 mx-5" id="form" on:submit=on_submit_callback>
        <div style="justify-content: center; display: flex;">
            <label class=upload_label for="file-input" id="uploadLabel">"choose alternative file"</label>
            <input _ref=file_input_ref id="file-input" type="file" class="upload" name="file-input" accept="application/pdf" on:change=submit_button_handler />
        </div>
        <br />
        <div style="justify-content: center; display: flex;">
            <input type="remove" id="remove-button" value="remove" class="uploadLabel display-box removeBox" on:click=remove_callback />
        </div>
        <div style="justify-content: center; display: flex;">
            <input type="view" id="view-button" value="view" class="uploadLabel display-box removeBox" />
        </div>
        <br />
        <div style="justify-content: center; display: flex;">
            <input type="submit" id="submit-button" value="submit" style=submit_button class=submit_button_class/>
        </div>
    </form>
    <br />
    <div>
        <section id="showcase" class="w-75 position-absolute start-50 translate-middle-x top-50 h-auto mx-5">
                {move || { pdf_path.value().get().map(|val| {
                view! {cx,
                <iframe _ref=iframe_ref id="iframe" width="100%" height="100%" frameborder="0" allowfullscreen src={
                    pdf_iframe_setter.set("/pdf/".to_string() + &val.unwrap().unwrap());
                pdf_iframe}/>
                }})}}
        </section>
    </div>
        <div style="height: 120vh"/>
        </Router>
    }
}

#[component]
fn ThumbnailList(
    cx: Scope,
    pdf_iframe_setter: WriteSignal<String>,
    thumbnail_refresh: ReadSignal<bool>,
) -> impl IntoView {
    let (thumbnail_class, thumbnail_class_setter) = create_signal(
        cx,
        String::from(
            "position-fixed d-flex flex-column overflow-scroll hover-overlay text-align-center",
        ),
    );

    let thumbnail_list = create_local_resource(cx, thumbnail_refresh, move |_| async {
        render_thumbnails().await
    });

    let thumbnail_callback = move |event: MouseEvent| {
        let image = event.target().unwrap().unchecked_into::<HtmlImageElement>();
        let src = image.src();
        let id = src.split('/').last().unwrap().split('.').next().unwrap();
        pdf_iframe_setter.set(format!("/pdf/{id}"));
    };

    view! {cx,
        <section id="thumbnails" style="background-color: #212529; margin-top: 98px; min-height:100%; width: 150px"
            class=thumbnail_class>
            <Transition fallback=move|| view! {cx, "loading"}>
                {move ||
                    thumbnail_list.read(cx).map(|list|
                        view! {cx,
                           <For
                           each=move || list.clone().unwrap()
                           key=|thumbnail| thumbnail.to_owned()
                           view=move |cx, item| {
                             view! {
                               cx,
                                <div class="my-2 m-auto" id=format!("/thumbnails/{item}")>
                                   <img src=format!("/thumbnails/{item}")
                                on:load=move|_| thumbnail_class_setter.set("position-fixed d-flex
                                flex-column overflow-scroll hover-overlay text-align-center px-3".to_string())
                                on:click=thumbnail_callback
                                />
                                </div>
                             }
                           }
                         />
                        })
                    }
            </Transition>
        </section>
    }
}

#[server(ExampleFunc)]
async fn server_func() -> Result<bool, ServerFnError> {
    let upload_path = std::fs::read_dir("./upload/").unwrap();
    Ok(upload_path.count() > 1)
}

#[server(ThumbnailRenderer)]
async fn render_thumbnails() -> Result<Vec<String>, ServerFnError> {
    let upload_path = std::fs::read_dir("./upload/thumbnails/").unwrap();
    Ok(upload_path
        .into_iter()
        .map(|items| {
            let path = items.unwrap().path();
            path.file_name().unwrap().to_str().unwrap().to_string()
        })
        .collect::<Vec<_>>())
}
