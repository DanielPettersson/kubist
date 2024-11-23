use crate::roll_events::RollInput;
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{in_state, EventWriter, IntoSystemConfigs, KeyCode, Res};
use crate::GameState;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_listener.run_if(in_state(GameState::InGame)));
    }
}

fn keyboard_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<RollInput>,
) {
    if keyboard.just_released(KeyCode::ArrowRight) {
        event_writer.send(RollInput::Right);
    } else if keyboard.just_released(KeyCode::ArrowLeft) {
        event_writer.send(RollInput::Left);
    } else if keyboard.just_released(KeyCode::ArrowDown) {
        event_writer.send(RollInput::Down);
    } else if keyboard.just_released(KeyCode::ArrowUp) {
        event_writer.send(RollInput::Up);
    }
}
