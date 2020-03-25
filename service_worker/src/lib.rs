use gloo::events::EventListener;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::*;
use futures::prelude::*;

// pub struct WorkerManager {
//     oninstall: EventListener,
//     onmessage: EventListener,
//     onpush: EventListener,
//     onactivate: EventListener,
// }

#[wasm_bindgen(start)]
pub fn start() {
    let _ = femme::start(log::LevelFilter::Info);
}

#[wasm_bindgen]
// register service worker with caches.
pub async fn install_hook()  {
    let scope = js_sys::global().unchecked_into::<web_sys::ServiceWorkerGlobalScope>();
    log::info!("scope found");
    let caches = scope.caches().expect("to get cache");
    // let cache = JsFuture::from(caches.open("v1")).await.map(|res| res.unchecked_into::<web_sys::Cache>()).expect("to save cache");
    // let res = JsFuture::from(cache.add_all_with_str_sequence(&serde_wasm_bindgen::to_value(&["index.html"]).expect("unable to get array"))).await.expect("unable to cache all");
}

#[wasm_bindgen]
/// use to cleanup unused resources.
pub async fn activate_hook() {
    let scope = js_sys::global().unchecked_into::<web_sys::ServiceWorkerGlobalScope>();
    log::info!("scope activated");
}

pub async fn cache_handler(req: web_sys::Request) -> Option<web_sys::Response> {
    let scope = js_sys::global().unchecked_into::<web_sys::ServiceWorkerGlobalScope>();
    let cache = scope.caches().expect("to get cache");
    let cache_response = JsFuture::from(cache.match_with_request(&req)).await;
    match cache_response {
        Ok(resp) => {
            if let Ok(res) = resp.dyn_into::<web_sys::Response>() {
                log::info!("cache hit");
                Some(res)
            } else {
                log::info!("cache missed");
                None
            }

        }
        _ => {
            log::info!("unable to open cache storage");
            None
        }
    }
    
}


#[wasm_bindgen]
pub async fn fetch_handler(req: web_sys::Request) -> Option<web_sys::Response> {
    cache_handler(req).await
}



