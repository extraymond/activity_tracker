use crate::elements::activity;
use crate::elements::dashboard;

use afterglow::prelude::*;

#[derive(Clone)]
pub enum ActivityHub {
    /// dashboard select on activity.
    NewEvent,
    RemoveSelected,
    Finished,
    OpenDialogue,
    RemoveFinished,
    StartExiting,
}
