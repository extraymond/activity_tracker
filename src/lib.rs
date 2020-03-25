#![recursion_limit = "512"]

use afterglow::prelude::*;
use cfg_if::cfg_if;
pub mod app;
pub mod elements;
pub mod msg_bus;
pub mod pages;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }

}

#[wasm_bindgen]
pub fn start() {
    if cfg!(feature = "console_error_panic_hook") {
        console_error_panic_hook::set_once();
    }

    if cfg!(debug_assertions) {
        femme::start(log::LevelFilter::Debug).expect("unable to start logger");
        log::info!("debug build");
    } else {
        femme::start(log::LevelFilter::Warn).expect("unable to start logger");
    }

    Entry::init_app::<pages::home::model::Model, pages::home::view::View>(None);
}
