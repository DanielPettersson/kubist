use crate::roll_events::RollEvent;
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{EventWriter, KeyCode, Res};

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_listener);
    }
}

fn keyboard_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<RollEvent>,
) {
    if keyboard.just_released(KeyCode::ArrowRight) {
        event_writer.send(RollEvent::Right);
    } else if keyboard.just_released(KeyCode::ArrowLeft) {
        event_writer.send(RollEvent::Left);
    } else if keyboard.just_released(KeyCode::ArrowDown) {
        event_writer.send(RollEvent::Down);
    } else if keyboard.just_released(KeyCode::ArrowUp) {
        event_writer.send(RollEvent::Up);
    }
}
