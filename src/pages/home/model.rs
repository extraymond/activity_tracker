use crate::elements::dashboard;
use afterglow::prelude::*;
use async_executors::*;

pub struct Model {
    pub head: HeadContainer,
    pub body: Container<dashboard::model::Model>,
    pub footer: FooterContainer,
    bus: BusService<crate::msg_bus::ActivityHub>,
}

#[derive(Clone)]
pub enum PageBus {}

pub struct HeadContainer;
pub struct BodyContainer;
pub struct FooterContainer;

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let head = HeadContainer;
        let bus = BusService::<crate::msg_bus::ActivityHub>::new();
        let mut data = dashboard::model::Model::new(render_tx.clone());
        data.bus = bus.clone();
        let body = Container::new(data, Box::new(dashboard::view::View::default()), render_tx);
        bus.register(body.sender.clone());
        let footer = FooterContainer;

        Model {
            head,
            body,
            footer,
            bus,
        }
    }

    fn mounted(
        sender: &MessageSender<Self>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
        handlers: &mut Vec<EventListener>,
    ) {
        spawn_local(Event::InitBus.dispatch(&sender));
    }
}

pub enum Event {
    InitBus,
    RequestNew,
    RequestRemove,
    RequestCleanup,
}

impl Into<Option<Message<Model>>> for crate::msg_bus::ActivityHub {
    fn into(self) -> Option<Message<Model>> {
        match self {
            crate::msg_bus::ActivityHub::Finished => {
                let notify = web_sys::Notification::new("an action has completed")
                    .expect("unable to notify");
                let (tx, rx) = oneshot::channel::<()>();
                let onclick = EventListener::once(
                    notify.unchecked_ref::<web_sys::EventTarget>(),
                    "click",
                    |_| {
                        let _ = tx.send(());
                    },
                );
                let executor = Bindgen::new();
                let fut_clicked = executor
                    .spawn_handle_local(async {
                        let _ = rx.await;
                        log::info!("action clicked");
                        drop(onclick);
                    })
                    .unwrap();

                let (tx, rx) = oneshot::channel::<()>();
                let onclose = EventListener::once(
                    notify.unchecked_ref::<web_sys::EventTarget>(),
                    "click",
                    |_| {
                        let _ = tx.send(());
                    },
                );
                let fut_closed = executor
                    .spawn_handle_local(async {
                        let _ = rx.await;
                        log::info!("action closed");
                        drop(onclose);
                    })
                    .unwrap();

                let manual_close = executor
                    .spawn_handle_local(async move {
                        use futures_timer::Delay;
                        use std::time::Duration;
                        Delay::new(Duration::from_secs(5)).await;
                        notify.close();
                        log::info!("clean up after 5 secs");
                    })
                    .unwrap();
                let combined = future::select_all(vec![fut_clicked, fut_closed, manual_close]);
                let combined = executor.spawn_handle_local(combined).unwrap();
                spawn_local(async {
                    combined.await;
                });
            }
            _ => {}
        }
        None
    }
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
            Event::InitBus => {
                target.bus.register(sender.clone());
            }
            Event::RequestNew => target.bus.publish(crate::msg_bus::ActivityHub::NewEvent),
            Event::RequestRemove => target
                .bus
                .publish(crate::msg_bus::ActivityHub::RemoveSelected),
            Event::RequestCleanup => target
                .bus
                .publish(crate::msg_bus::ActivityHub::RemoveFinished),
        }

        false
    }
}
