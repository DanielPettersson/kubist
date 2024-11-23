use bevy::app::{App, Plugin};
use bevy::prelude::Event;

pub struct RollEventsPlugin;

impl Plugin for RollEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RollEvent>();
    }
}
#[derive(Event, Debug)]
pub enum RollEvent {
    Right,
    Left,
    Up,
    Down,
}

impl RollEvent {
    pub fn get_id(&self) -> u64 {
        match self {
            RollEvent::Right => 1,
            RollEvent::Left => 2,
            RollEvent::Up => 3,
            RollEvent::Down => 4
        }
    }
    
    pub fn from_id(id: u64) -> Self {
        match id { 
            1 => RollEvent::Right,
            2 => RollEvent::Left,
            3 => RollEvent::Up,
            4 => RollEvent::Down,
            id => panic!("Undefined roll event {id}")
        }
    }
}