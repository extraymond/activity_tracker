use afterglow::prelude::*;

pub async fn init_service_worker() {
    let navigator = web_sys::window().unwrap().navigator();
    let sw_container = navigator.service_worker();
    let fut = JsFuture::from(sw_container.register("./sw.js"));
    match fut.await {
        Ok(res) => {
            let registrator = res.unchecked_into::<web_sys::ServiceWorkerRegistration>();
            log::info!("service worker up!!!");
        }
        Err(e) => {
            log::warn!("{:?}", e);
        }
    }
}

pub async fn init_notification() {
    let permission = JsFuture::from(
        web_sys::Notification::request_permission().expect("unable to request for permission"),
    )
    .await;

    if permission.is_ok() {
        match web_sys::Notification::permission() {
            web_sys::NotificationPermission::Granted => {
                let _ = web_sys::Notification::new("notified!!!");
            }
            _ => {
                log::trace!("not granted permission to show notification");
            }
        }
    }
}
