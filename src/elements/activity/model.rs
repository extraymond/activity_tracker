use afterglow::prelude::*;
use chrono::prelude::*;
use futures::lock::Mutex;
use futures_timer::Delay;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;
use strum_macros::Display;

pub struct Activity {
    pub start: DateTime<Utc>,
    pub now: DateTime<Utc>,
    pub end: Option<f32>,
    pub status: Status,
    pub tags: HashSet<String>,
    /// pause on 0, proceed on 1, cancel on 2.
    pub can_go: Option<Rc<Mutex<u8>>>,
    pub elasped: Option<chrono::Duration>,
    pub bus: BusService<crate::msg_bus::ActivityHub>,
    goal: usize,
}

#[derive(Display)]
pub enum Status {
    Started,
    Running,
    Completed,
    Paused,
    Exiting,
}

impl LifeCycle for Activity {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let now = Utc::now();
        let bus = BusService::new();

        Activity {
            start: now,
            now,
            status: Status::Paused,
            end: None,
            tags: HashSet::new(),
            elasped: None,
            can_go: None,
            bus,
            goal: 5,
        }
    }

    fn mounted(
        sender: &MessageSender<Self>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
        handlers: &mut Vec<EventListener>,
    ) {
        spawn_local(Event::Started(true).dispatch(&sender));
    }

    fn destroyed(
        &self,
        sender: &MessageSender<Self>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) {
        // destroy timer too
        spawn_local(Event::Destoryed.dispatch(&sender));
    }
}

pub enum Event {
    TimeUpdated(DateTime<Utc>),
    ElaspedUpdated(chrono::Duration),
    Started(bool),
    Paused,
    Continue,
    Completed,
    Clicked,
    Destoryed,
    TriggerAnimation,
}

impl Into<Option<Message<Activity>>> for crate::msg_bus::ActivityHub {
    fn into(self) -> Option<Message<Activity>> {
        match self {
            crate::msg_bus::ActivityHub::StartExiting => {
                return Some(Box::new(Event::TriggerAnimation))
            }
            _ => {}
        }
        None
    }
}

impl Messenger for Event {
    type Target = Activity;

    fn update(
        &self,
        target: &mut Self::Target,
        sender: &MessageSender<Self::Target>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) -> bool {
        match self {
            Event::Started(autostart) => {
                target.status = Status::Started;
                let signal = Rc::new(Mutex::new(0));
                let signal_handle = signal.clone();
                target.can_go = Some(signal);
                let sender = sender.clone();
                if *autostart {
                    target.status = Status::Running;
                }
                let autostart = autostart.clone();

                let task = async move {
                    if autostart {
                        let mut signal_val = signal_handle.lock().await;
                        *signal_val = 1_u8;
                    }
                    loop {
                        let chunk_start = Utc::now();

                        Delay::new(Duration::from_millis(100)).await;
                        let signal_val = signal_handle.lock().await;
                        match *signal_val {
                            1 => {
                                let chunk_end = Utc::now();
                                Event::TimeUpdated(chunk_end).dispatch(&sender).await;
                                Event::ElaspedUpdated(chunk_end - chunk_start)
                                    .dispatch(&sender)
                                    .await;
                            }
                            2 => break,
                            _ => {}
                        }
                    }
                };
                spawn_local(task);
                return true;
            }
            Event::Paused => {
                target.status = Status::Paused;
                if let Some(signal) = target.can_go.as_ref() {
                    let signal_handle = signal.clone();
                    spawn_local(async move {
                        let mut signal = signal_handle.lock().await;
                        *signal = 0;
                    });
                    return true;
                }
            }
            Event::Continue => {
                target.status = Status::Running;
                if let Some(signal) = target.can_go.as_ref() {
                    let signal_handle = signal.clone();
                    spawn_local(async move {
                        let mut signal = signal_handle.lock().await;
                        *signal = 1;
                    });
                    return true;
                }
            }
            Event::Completed => {
                target.status = Status::Completed;
                if let Some(signal) = target.can_go.as_ref() {
                    let signal_handle = signal.clone();
                    spawn_local(async move {
                        let mut signal = signal_handle.lock().await;
                        *signal = 2;
                    });
                    return true;
                }
            }
            Event::TimeUpdated(time) => {
                target.now = *time;
                return true;
            }
            Event::Clicked => match target.status {
                Status::Started | Status::Paused => {
                    spawn_local(Event::Continue.dispatch(&sender));
                }
                Status::Running => {
                    spawn_local(Event::Paused.dispatch(&sender));
                }
                Status::Completed | Status::Exiting => {}
            },
            Event::Destoryed => {
                if let Some(signal) = target.can_go.as_ref() {
                    let signal_handle = signal.clone();
                    spawn_local(async move {
                        let mut signal = signal_handle.lock().await;
                        *signal = 2;
                    });
                }
            }
            Event::ElaspedUpdated(duration) => {
                if let Some(time) = target.elasped.as_mut() {
                    return time
                        .checked_add(duration)
                        .map(|val| {
                            if val.num_seconds() as usize == target.goal {
                                target.bus.publish(crate::msg_bus::ActivityHub::Finished);
                                let sender = sender.clone();
                                spawn_local(async move {
                                    Event::Completed.dispatch(&sender).await;
                                });
                            }
                            target.elasped.replace(val);

                            true
                        })
                        .unwrap_or_default();
                } else {
                    return target
                        .elasped
                        .replace(*duration)
                        .map(|_| true)
                        .unwrap_or_default();
                }
            }
            Event::TriggerAnimation => {
                if let Status::Completed = target.status {
                    log::info!("animation triggered");
                    target.status = Status::Exiting;
                    return true;
                }
            }
        }

        false
    }
}
