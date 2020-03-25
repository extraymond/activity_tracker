use afterglow::prelude::*;
use futures_timer::Delay;
use std::time::Duration;

pub struct Model {
    pub active: bool,
    pub bus: BusService<crate::msg_bus::ActivityHub>,
    pub visible: f32,
}

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let bus = BusService::<crate::msg_bus::ActivityHub>::new();
        Model {
            active: false,
            bus,
            visible: 0_f32,
        }
    }
}

pub enum Event {
    Open,
    OpenWithAnimation(u64),
    CloseWithAnimation(u64),
    VisibleChanged(f32),
    Close,
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
            Event::Open => {
                target.active = true;
            }
            Event::Close => {
                target.active = false;
            }
            Event::OpenWithAnimation(time) => {
                let time = time.clone();
                let sender = sender.clone();
                let fut = async move {
                    Event::VisibleChanged(0_f32).dispatch(&sender).await;
                    Event::Open.dispatch(&sender).await;
                    // Let macrotask queue advance so browser have chance to detect css transition
                    Delay::new(Duration::from_millis(10)).await;
                    Event::VisibleChanged(1_f32).dispatch(&sender).await;
                };
                spawn_local(fut);
                return false;
            }
            Event::CloseWithAnimation(time) => {
                let time = time.clone();
                let sender = sender.clone();
                let fut = async move {
                    Event::VisibleChanged(0_f32).dispatch(&sender).await;
                    // Let macrotask queue advance so browser have chance to detect css transition
                    Delay::new(Duration::from_millis(time)).await;
                    Event::Close.dispatch(&sender).await;
                };
                spawn_local(fut);
                return false;
            }
            Event::VisibleChanged(active) => {
                target.visible = *active;
                return true;
            }
        }
        true
    }
}

type BusMsg = crate::msg_bus::ActivityHub;
impl Into<Option<Message<Model>>> for BusMsg {
    fn into(self) -> Option<Message<Model>> {
        match self {
            BusMsg::OpenDialogue => return Some(Box::new(Event::OpenWithAnimation(500))),
            _ => {}
        }
        None
    }
}
