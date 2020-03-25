use afterglow::prelude::*;

pub struct Model {
    pub status: web_sys::NotificationPermission,
}

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let status = web_sys::Notification::permission();
        Model { status }
    }
}

pub enum Event {
    Request,
    GetRequest,
}

impl Messenger for Event {
    type Target = Model;

    fn update(
        &self,
        target: &mut Self::Target,
        sender: &MessageSender<Self::Target>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) -> bool {
        match self {
            Event::Request => {
                let sender = sender.clone();
                let fut = async move {
                    if let Ok(promise) = web_sys::Notification::request_permission() {
                        let fut = JsFuture::from(promise).await;
                        if fut.is_ok() {
                            Event::GetRequest.dispatch(&sender).await;
                        }
                    }
                };
                spawn_local(fut);
            }
            Event::GetRequest => {
                target.status = web_sys::Notification::permission();
                return true;
            }
        }
        false
    }
}
