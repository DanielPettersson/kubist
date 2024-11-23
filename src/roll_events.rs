use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Event, Vec3};

pub struct RollEventsPlugin;

impl Plugin for RollEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RollEvent>().add_event::<RollInput>();
    }
}

#[derive(Event, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RollInput {
    Right,
    Left,
    Up,
    Down,
}

impl RollInput {
    pub fn to_roll_event(&self, entity: Entity) -> RollEvent {
        match self {
            RollInput::Right => RollEvent::Right(entity),
            RollInput::Left => RollEvent::Left(entity),
            RollInput::Up => RollEvent::Up(entity),
            RollInput::Down => RollEvent::Down(entity)
        }
    }
    
    pub fn is_opposite(&self, roll_input: RollInput) -> bool {
        match self {
            RollInput::Right => roll_input == RollInput::Left,
            RollInput::Left => roll_input == RollInput::Right,
            RollInput::Up => roll_input == RollInput::Down,
            RollInput::Down => roll_input == RollInput::Up,
        }
    }
}

#[derive(Event)]
pub enum RollEvent {
    Right(Entity),
    Left(Entity),
    Up(Entity),
    Down(Entity),
}

impl RollEvent {
    pub fn get_id(&self) -> u64 {
        match self {
            RollEvent::Right(_) => 1,
            RollEvent::Left(_) => 2,
            RollEvent::Up(_) => 3,
            RollEvent::Down(_) => 4,
        }
    }

    pub fn get_entity(&self) -> &Entity {
        match self {
            RollEvent::Right(e) => e,
            RollEvent::Left(e) => e,
            RollEvent::Up(e) => e,
            RollEvent::Down(e) => e,
        }
    }

    pub fn from_id(id: u64, entity: Entity) -> Self {
        match id {
            1 => RollEvent::Right(entity),
            2 => RollEvent::Left(entity),
            3 => RollEvent::Up(entity),
            4 => RollEvent::Down(entity),
            id => panic!("Undefined roll event {id}"),
        }
    }

    pub fn roll_translation(&self) -> Vec3 {
        match self {
            RollEvent::Right(_) => Vec3::new(0.5, 0.0, 0.0),
            RollEvent::Left(_) => Vec3::new(-0.5, 0.0, 0.0),
            RollEvent::Up(_) => Vec3::new(0.0, 0.5, 0.0),
            RollEvent::Down(_) => Vec3::new(0.0, -0.5, 0.0),
        }
    }

    pub fn roll_axis(&self) -> Vec3 {
        match self {
            RollEvent::Right(_) => Vec3::Y,
            RollEvent::Left(_) => Vec3::NEG_Y,
            RollEvent::Up(_) => Vec3::NEG_X,
            RollEvent::Down(_) => Vec3::X,
        }
    }
}
