use crate::elements::activity;
use afterglow::prelude::*;

pub struct Model {
    pub activities: Vec<Container<activity::model::Activity>>,
    pub selected: Option<usize>,
    pub bus: BusService<crate::msg_bus::ActivityHub>,
}

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let bus = BusService::<crate::msg_bus::ActivityHub>::new();

        Model {
            activities: vec![],
            selected: None,
            bus,
        }
    }
}

pub enum Event {
    AddActivity,
    RemoveActivity,
    Selected(usize),
    InitBus,
    OpenDialogue,
    Cleanup,
    RemoveDead(Vec<usize>),
    DeleteActivity(usize),
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
            Event::OpenDialogue => {
                target.bus.publish(BusMsg::OpenDialogue);
            }
            Event::InitBus => {
                target.bus.register(sender.clone());
            }
            Event::AddActivity => {
                let mut data = activity::model::Activity::new(render_tx.clone());
                data.bus = target.bus.clone();
                let new_activity =
                    Container::new(data, Box::new(activity::view::View), render_tx.clone());
                target.bus.register(new_activity.sender.clone());
                target.activities.push(new_activity);
                return true;
            }
            Event::RemoveActivity => {
                if let Some(idx) = target.selected {
                    if target.activities.len() - 1 == idx {
                        target.selected = None;
                    }
                    target.activities.remove(idx);
                    return true;
                }
            }
            Event::DeleteActivity(idx) => {
                if target.selected == Some(*idx) {
                    target.selected = None;
                }
                target.activities.remove(*idx);
                return true;
            }
            Event::Selected(id) => {
                if let Some(old_id) = target.selected {
                    if id == &old_id {
                        target.selected = None;
                        return true;
                    }
                }
                target.selected.replace(*id);
                return true;
            }
            Event::Cleanup => {
                let children = target
                    .activities
                    .iter()
                    .map(|child| (child.sender.clone(), child.data.clone()))
                    .collect::<Vec<_>>();
                let sender = sender.clone();
                let mut bus_tx = target.bus.bus_tx.clone();
                spawn_local(async move {
                    let mut dead_ids = vec![];
                    for (idx, (sender, child)) in children.iter().enumerate() {
                        let child = child.lock().await;
                        if let crate::elements::activity::model::Status::Completed = child.status {
                            dead_ids.push(idx);
                        }
                    }
                    if !dead_ids.is_empty() {
                        Event::RemoveDead(dead_ids).dispatch(&sender).await;
                    }
                });
                return true;
            }
            Event::RemoveDead(dead_ids) => {
                if !dead_ids.is_empty() {
                    let items = dead_ids
                        .iter()
                        .rev()
                        .map(|idx| (idx.clone(), target.activities.get(*idx)))
                        .filter(|val| val.1.is_some())
                        .map(|(idx, child)| child.unwrap().sender.clone())
                        .collect::<Vec<_>>();

                    let ids = dead_ids.clone();

                    let sender = sender.clone();
                    spawn_local(async move {
                        stream::iter(items)
                            .for_each(|sender| async move {
                                crate::elements::activity::model::Event::TriggerAnimation
                                    .dispatch(&sender)
                                    .await;
                            })
                            .await;
                        use futures_timer::Delay;
                        use std::time::Duration;
                        Delay::new(Duration::from_secs(1)).await;
                        for idx in ids.iter().rev() {
                            Event::DeleteActivity(*idx).dispatch(&sender).await;
                        }
                    });
                }
            }
        }
        false
    }
}

type BusMsg = crate::msg_bus::ActivityHub;

impl Into<Option<Message<Model>>> for BusMsg {
    fn into(self) -> Option<Message<Model>> {
        log::info!("bus notification");
        match self {
            BusMsg::NewEvent => Some(Box::new(Event::AddActivity)),
            BusMsg::RemoveSelected => Some(Box::new(Event::RemoveActivity)),
            BusMsg::RemoveFinished => Some(Box::new(Event::Cleanup)),
            _ => None,
        }
    }
}
