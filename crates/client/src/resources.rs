use std::sync::mpsc::Sender;

use crate::screens::loading::LoadingState;

#[derive(Debug)]
pub struct Resources {}

impl Resources {
    pub fn load(loading_state_sender: Sender<LoadingState>) {
        loading_state_sender
            .send(LoadingState::Finish {
                resources: Resources {},
            })
            .expect("poisoned");
    }
}
