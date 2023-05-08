use crate::styles::navbar::*;
use gloo_events::{EventListener, EventListenerOptions, EventListenerPhase};
use gloo_net::http::Request;
use leptos::{
    html::{Input, Ul, A},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};
use web_sys::{HtmlDivElement, MouseEvent};

// #[cfg(feature = "ssr")]
#[component(transparent)]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view!(cx,
    <Router>
        <NavBarHead/>
        <header>
        <nav class="navbar navbar-dark bg-dark navbar-expand-lg ms-auto fixed-top">
            <div class="container">
                <div class="row navbar-brand p-3 ms-lg-auto">
                    <A href="/">
                        <img src="/surroundlogo.png" alt="surround_logo" width="30px" height="50px" />
                    </A>
                </div>
                <NavBarToggle/>
                <NavBarCollapse/>
            </div>
        </nav>
        // <script src="/navbar.js"></script>
    </header>
    </Router>
        )
}

#[component(transparent)]
pub fn NavBarHead(cx: Scope) -> impl IntoView {
    view! {cx,
            <head>
              <meta name="viewport" content="width=device-width, initial-scale=1" />
              <title>"Demo 1"</title>
              <Stylesheet href="/bootstrap-5.2.3-dist/css/bootstrap.min.css" />
              <Script src="/bootstrap-5.2.3-dist/js/bootstrap.bundle.min.js"/>
              <Stylesheet id="leptos" href="/fontawesome-free-6.2.1-web/css/all.min.css"/>
            </head>
    }
}

#[component(transparent)]
pub fn NavBarCollapse(cx: Scope) -> impl IntoView {
    let search = create_node_ref::<A>(cx);
    let style = navbar_style();
    let dropdown_menu_node = create_node_ref::<Ul>(cx);
    let (dropdown_menu, dropdown_menu_setter) = create_signal(cx, String::from("dropdown-menu"));
    let (list, list_setter) = create_signal(cx, view! {cx, <div/>});

    let search_callback = move |_: MouseEvent| {
        // let target = search.get().unwrap().next_sibling().unwrap();
        let target = document()
            .get_elements_by_class_name("form-control me-2")
            .item(0)
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        log!("{:?}", target.value());

        let options = EventListenerOptions {
            phase: EventListenerPhase::Bubble,
            passive: false,
        };

        let _ = EventListener::new_with_options(&target, "change", options ,move |event| {
            let input_value = event_target::<HtmlInputElement>(event).value();
            // let input_value = document().get_elements_by_class_name("form-control me-2").item(0).unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
            log!("here");

            match input_value.trim().len().gt(&0) {
                true => spawn_local(async move {
                    log!("here");
                    let xhr = Request::new("/headings")
                        .body(input_value)
                        .send()
                        .await
                        .unwrap();

                    if xhr.ok() && xhr.text().await.unwrap() != "" {
                        dropdown_menu_setter
                            .update(|value: &mut String| value.push_str("display-block"));
                        let options = xhr.text().await.unwrap();

                        let options: Vec<String> =
                            options.split('\n').map(|s| s.to_string()).collect();

                        list_setter.set(view! {cx,
                            <div>
                           <For
                           each=move || options.clone()
                           key=|option| option.to_owned()
                           view=move |cx, option| {
                             view! {
                               cx,
                                        <li class="p-2"><a href=option class="py-2 text-muted"/></li>
                                <div class="dropdown-divider"/>
                             }
                           }
                         />
                            </div>
                        });
                    }
                }),
                false => {
                    dropdown_menu_setter
                        .update(|value: &mut String| *value = value.replace("display-block", ""));
                }
            }
        });
    };
    styled::view! {cx, style,
               <div class="collapse navbar-collapse" id="navbarSupportedContent"> <ul class="navbar-nav nav ms-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <A class="nav-link active"
                                href="https://www.surroundaustralia.com/">"Main"</A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="https://www.surroundaustralia.com/about-us">"About Us"</A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link me-1" href="https://www.surroundaustralia.com/contact-us">"Contact Us"</A>
                        </li>

                        <li class="nav-item mb-auto" on:click=search_callback>
                            <a _ref=search class="btn btn-dark" data-bs-toggle="collapse" href="#searchBarCollapse" role="button"
                                aria-expanded="false" aria-controls="searchBarCollapse" id="search"><i
                                    class="fa fa-search"></i></a>
                        </li>
                        <Dropdown dropdown_menu_signal=dropdown_menu dropdown_menu_node list/>
                    </ul>
                </div>
    }
}

#[component(transparent)]
pub fn Dropdown(
    cx: leptos::Scope,
    dropdown_menu_signal: ReadSignal<String>,
    dropdown_menu_node: NodeRef<Ul>,
    list: ReadSignal<HtmlElement<leptos::html::Div>>,
) -> impl IntoView {
    view! {cx,
        <div class="dropdown-center">
            <div class="collapse searchCollapse" id="searchBarCollapse">
                <form class="d-flex" role="search">
                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"
                        aria-expanded="false"/>
                    <button class="btn btn-dark" type="submit">"Search"</button>
                </form>
                <ul class=dropdown_menu_signal _ref=dropdown_menu_node>
                {list}
                </ul>
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn NavBarToggle(cx: Scope) -> impl IntoView {
    view! {cx,
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse"
         data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent"
         aria-expanded="false" aria-label="Toggle navigation">
         <span class="navbar-toggler-icon"></span>
    </button>
    }
}
