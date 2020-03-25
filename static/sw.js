importScripts("./service_worker.js");

const {
    install_hook,
    activate_hook,
    fetch_handler
} = wasm_bindgen;


let self = this;

this.addEventListener("install", e => {
    e.waitUntil(
        caches.open("v1").then(cache => {
            cache.addAll([
                "./index.html",
                "./activity_tracker.js",
                "./activity_tracker_bg.wasm",
                "https://cdn.jsdelivr.net/npm/bulma@0.8.0/css/bulma.min.css",
                "./style.css"
            ])
        })
    )
})

// this.addEventListener("activate", e=> {
//     e.waitUntil()
// })

this.addEventListener("push", e => {
    self.registration.showNotification("hello");
})

this.addEventListener("fetch", e => {
    e.respondWith(req_handler(e.request))
})

async function req_handler(req) {
    let cache_resp = await caches.match(req);
    if (cache_resp != null) {
        return cache_resp
    } else {
        return fetch(req)
    }
}