use crate::colour::Colour;
use crate::events::{Button, EventTask};

///
/// Common controller behaviours
///
pub trait Controller: EventTask {
    ///
    /// Set the State of an Button LED
    ///
    /// **Arguments**
    /// - button - Button associated with a LED
    /// - colour - Colour to apply
    fn set_button_led(&mut self, button: Button, colour: Colour);

    ///
    /// Set the State of an Pad LED
    ///
    /// **Arguments**
    /// - pad - Pad number
    /// - colour - Colour to apply
    fn set_pad_led(&mut self, pad: u8, colour: Colour);
}
