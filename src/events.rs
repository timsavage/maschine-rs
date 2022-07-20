use crate::error::Error;
use std::collections::VecDeque;

///
/// System Events
///
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Event {
    ///
    /// Button change (Button, Pressed, Shift)
    ///
    Button(Button, bool, bool),

    ///
    /// Encoder change (Encoder Number, Direction, Shift)
    ///
    Encoder(u8, Direction, bool),

    ///
    /// Pad change (Pad Number, Velocity, Shift)
    ///
    PadC(u8, u8, bool),
}

///
/// Direction of encoder
///
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
}

///
/// Button Identifiers
///
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Button {
    Erase,
    Rec,
    Play,
    Grid,
    TransportRight,
    TransportLeft,
    Restart,
    MainEncoder,
    NoteRepeat,
    Sampling,
    Browse,
    Group,
    Main,
    BrowseRight,
    BrowseLeft,
    Nav,
    Control,
    F3,
    F2,
    F1,
    Mute,
    Solo,
    Select,
    Duplicate,
    View,
    PadMode,
    Pattern,
    Scene,
    Unknown,
}

///
/// Context object for adding events
///
pub struct EventContext {
    pub events: VecDeque<Event>,
}

impl EventContext {
    pub fn new() -> Self {
        EventContext {
            events: VecDeque::new(),
        }
    }

    ///
    /// Add an event into the context
    ///
    pub fn add_event(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

///
/// Generator for events
///
pub trait EventTask {
    ///
    /// Perform any update events with the controller device
    ///
    fn tick(&mut self, context: &mut EventContext) -> Result<(), Error>;
}

/// Handle events
pub trait EventHandler {
    /// Handle event and return if it was handled.
    ///
    /// **Arguments**
    /// * `event` - Reference to the event to be handled
    ///
    fn handle(&mut self, event: &Event) -> bool;
}
