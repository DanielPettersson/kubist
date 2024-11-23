use crate::cube::{spawn_cube, RollingCubesCounter};
use crate::roll_events::{RollEvent, RollInput};
use crate::{GameState, SceneAssets};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Commands, Component, Entity, EventReader, EventWriter, IntoSystemConfigs, Local,
    NextState, OnEnter, Query, Res, ResMut, Resource, Startup,
};
use rand::Rng;

pub struct BoardPlugin;

const BOARD_SIZE: usize = 4;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(OnEnter(GameState::StartGame), setup)
            .add_systems(Update, shuffle.run_if(in_state(GameState::StartGame)))
            .add_systems(Update, roll);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(ShuffleCounter(0));
}

fn setup(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut shuffle_counter: ResMut<ShuffleCounter>,
) {
    shuffle_counter.0 += 0;

    let mut board = Board {
        cubes: [[None; BOARD_SIZE]; BOARD_SIZE],
    };

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if x != 1 || y != 1 {
                board.cubes[y][x] = Some(spawn_cube(
                    &mut commands,
                    scene_assets.rubiks_cube.clone(),
                    x as f32 - BOARD_SIZE as f32 / 2.0 + 0.5,
                    y as f32 - BOARD_SIZE as f32 / 2.0 + 0.5,
                ));
            }
        }
    }

    commands.spawn(board);
}

fn shuffle(
    rolling_cubes_counter: Res<RollingCubesCounter>,
    mut roll_inputs: EventWriter<RollInput>,
    mut shuffle_counter: ResMut<ShuffleCounter>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_roll: Local<Option<RollInput>>,
) {
    if rolling_cubes_counter.0 == 0 {
        let roll_input = random_roll(*last_roll);
        roll_inputs.send(roll_input);

        shuffle_counter.0 += 1;
        *last_roll = Some(roll_input);
    }

    if shuffle_counter.0 > 50 {
        next_state.set(GameState::InGame);
    }
}

fn random_roll(last_roll_opt: Option<RollInput>) -> RollInput {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..4);
    let roll = match i {
        0 => RollInput::Right,
        1 => RollInput::Left,
        2 => RollInput::Down,
        3 => RollInput::Up,
        _ => unreachable!(),
    };

    if let Some(last_roll) = last_roll_opt {
        if last_roll.is_opposite(roll) {
            return random_roll(last_roll_opt);
        }
    }
    roll
}

fn roll(
    mut roll_inputs: EventReader<RollInput>,
    mut roll_events: EventWriter<RollEvent>,
    mut query_board: Query<&mut Board>,
    rolling_cubes_counter: Res<RollingCubesCounter>,
) {
    if rolling_cubes_counter.0 != 0 {
        return;
    }

    for roll_input in roll_inputs.read() {
        for mut board in query_board.iter_mut() {
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    let pos = BoardPos::new(x, y);
                    if let None = board.get_cube(pos) {
                        if let Some(roll_from_pos) = board.get_roll_from_pos(pos, roll_input) {
                            if let Some(roll_cube) = board.get_cube(roll_from_pos) {
                                roll_events.send(roll_input.to_roll_event(roll_cube));
                                board.set_cube(roll_from_pos, None);
                                board.set_cube(pos, Some(roll_cube));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Board {
    cubes: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn get_roll_from_pos(&self, pos: BoardPos, roll_input: &RollInput) -> Option<BoardPos> {
        let delta: (i32, i32) = match roll_input {
            RollInput::Right => (-1, 0),
            RollInput::Left => (1, 0),
            RollInput::Up => (0, -1),
            RollInput::Down => (0, 1),
        };
        let x = pos.x as i32 + delta.0;
        let y = pos.y as i32 + delta.1;

        if x < 0 || x >= BOARD_SIZE as i32 || y < 0 || y >= BOARD_SIZE as i32 {
            return None;
        }
        Some(BoardPos {
            x: x as usize,
            y: y as usize,
        })
    }

    fn get_cube(&self, pos: BoardPos) -> Option<Entity> {
        self.cubes[pos.y][pos.x]
    }

    fn set_cube(&mut self, pos: BoardPos, entity: Option<Entity>) {
        self.cubes[pos.y][pos.x] = entity;
    }
}

#[derive(Clone, Copy, Debug)]
struct BoardPos {
    x: usize,
    y: usize,
}

impl BoardPos {
    fn new(x: usize, y: usize) -> Self {
        BoardPos { x, y }
    }
}

#[derive(Resource)]
struct ShuffleCounter(pub usize);
