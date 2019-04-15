use amethyst::shrev::{EventChannel};
use crate::network::InputEvent;

pub struct OwnedInput {
    pub is_server: bool,
    pub input: InputEvent,
}
pub type OrderedInput = EventChannel<OwnedInput>;

