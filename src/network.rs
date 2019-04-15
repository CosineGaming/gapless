use amethyst::core::math::Vector2;

/// Any event that could be sent is serialized ultimately to a CustomNetEvent
/// This includes some common fields to all events
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CustomNetEvent {
    pub frame: u64,
    pub from_server: bool,
    pub event: AnyEvent,
}
/// Variable parts of events are broken down by AnyEvent
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum AnyEvent {
    Input(InputEvent),
    Update(AnyUpdate),
}
/// An Input is sent as "what keys are held down" - they're sent *reliably*
/// so they will always be received
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct InputEvent {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
/// UpdateEvents are sent only occasionally and unreliably,
/// because *their only purpose is to combat two things*:
/// - Simulation non-determinism (virtually unavoidable)
/// - Latency non-determinism (unavoidable)
/// *No data is meant to be sent by UpdateEvents, it's only a corrective force*
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum AnyUpdate {
    Client(ClientUpdate),
    Server(ServerUpdate),
}
/// Client update is only concerned with client-owned data (player, etc)
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ClientUpdate {
    pub player_pos: Vector2<f32>,
}
/// Server update IS ALSO CONCERNED with player-owned data because server is a player
/// But is also concerned with data that would otherwise have no owner
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ServerUpdate {
    pub player_pos: Vector2<f32>,
}

/// TODO: use some actual ID or something
#[derive(Clone, Debug)]
pub struct NetParams {
    pub is_server: bool,
    // Because the simulation doesn't start at the same time the true frame
    // number must be determined by connection time
    pub first_frame: u64,
}

