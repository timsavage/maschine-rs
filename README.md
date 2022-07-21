# Maschine

This crate for interacting with a Native Instruments Maschine interface over USB.

These devices have a large number of buttons, velocity sensitive pads, rotary encoder and Dot-matrix LCD.

Currently, only the *Maschine Mikro mk2* is supported.

## Usage

## Example

```rust
extern crate hidapi;

use hidapi::HidApi;
use maschine::{get_device, Canvas, Colour, Controller, Event, EventContext, EventTask};


fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();

    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            match event {
                Event::Button(button, pressed, shift) => {
                    if pressed {
                        // If button is pressed assign it a random colour
                        ctlr.set_button_led(button, Colour::random());
                        println!("{:?}", button);
                    } else {
                        // Turn the LED off on release.
                        ctlr.set_button_led(button, Colour::BLACK);
                    }
                }
                // Print out other events
                _ => println!("{:?}", event),
            }
        }
    }
}

```
