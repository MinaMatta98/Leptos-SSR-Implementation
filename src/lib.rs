#![feature(result_flattening)]
mod functions;
mod pages;
mod styles;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use leptos::*;
      use functions::*;

      // initializes logging using the `log` crate
        // tracing::debug!("loading hdrate");
      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(|cx|
    view! {cx,
        <RenderIndex/>
    });
}
}
}
