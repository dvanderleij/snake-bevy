use crate::GameState;
use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
    time::common_conditions::on_timer,
};
use rand::Rng;
use std::time::Duration;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                PostStartup,
                sprite_movement.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                PreUpdate,
                (
                    handle_input.run_if(in_state(GameState::Game)),
                    spawn_rewards
                        .run_if(in_state(GameState::Game))
                        .run_if(on_timer(Duration::from_secs_f32(5.0))),
                ),
            )
            .add_systems(
                Update,
                process_position
                    .run_if(on_timer(Duration::from_secs_f32(0.2)))
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                PostUpdate,
                sprite_movement
                    .run_if(on_timer(Duration::from_secs_f32(0.2)))
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_game);
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Game;

impl Direction {
    fn next_coordinates(&self, coordinates: Coordinates) -> Coordinates {
        let x = coordinates.0 .0;
        let y = coordinates.0 .1;
        Coordinates::from(match self {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        })
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn new(input: KeyCode) -> Option<Self> {
        match input {
            KeyCode::W | KeyCode::Up => Some(Direction::Up),
            KeyCode::A | KeyCode::Left => Some(Direction::Left),
            KeyCode::S | KeyCode::Right => Some(Direction::Down),
            KeyCode::D | KeyCode::Down => Some(Direction::Right),
            _ => None,
        }
    }

    fn change(input: Res<Input<KeyCode>>) -> Option<Direction> {
        input
            .get_pressed()
            .find_map(|key_code| Direction::new(*key_code))
    }
}

#[derive(Component)]
enum TileType {
    Reward,
    GameOver,
}

fn create_square() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
            ..default()
        },
        ..default()
    }
}

#[derive(Component)]
struct RequestedDirection(Direction);

const START: i16 = -10;
const END: i16 = 10;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Game)
        .insert(SpatialBundle::default())
        .with_children(|commands| {
            let initial_direction = Direction::Up;
            commands
                .spawn(initial_direction)
                .insert(Coordinates((0, 0)))
                .insert(create_square());

            for i in START..=END {
                // Spawns double tile in corners, doesn't really matter
                for coordinates in [(START, i), (i, START), (END, i), (i, END)] {
                    commands
                        .spawn(TileType::GameOver)
                        .insert(Coordinates::from(coordinates))
                        .insert(create_square());
                }
            }
            commands.spawn(RequestedDirection(initial_direction));
            commands.spawn(TailOrder { segments: vec![] });
            let sound_handle = asset_server.load("background_music.wav");
            commands.spawn(BackgroundMusic).insert(AudioBundle {
                source: sound_handle,
                settings: PlaybackSettings::LOOP,
            });
        });
}

#[derive(Component)]
struct BackgroundMusic;

#[derive(Component)]
struct TailOrder {
    segments: Vec<Entity>,
}

fn spawn_rewards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_query: Query<Entity, With<Game>>,
) {
    // TODO allow for seed? doesn't really matter unless this project is going to grow Didn't do it beacuse of threadlocal stuff
    // let seed = settings.seed;
    // let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rng = rand::thread_rng();
    const RANGE: i16 = END - 1;
    let coordinates =
        Coordinates::from((rng.gen_range(-RANGE..=RANGE), rng.gen_range(-RANGE..=RANGE)));
    commands
        .entity(game_query.single())
        .with_children(|commands| {
            commands
                .spawn(TileType::Reward)
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(SQUARE_SIZE / 4.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(translation_from_coordinates(
                        coordinates,
                    )),
                    ..default()
                })
                .insert(coordinates);
        });
}

const SQUARE_SIZE: f32 = 25.;
fn translation_from_coordinates(coordinates: Coordinates) -> Vec3 {
    let inner: (i16, i16) = coordinates.0;
    Vec3::new(
        inner.0 as f32 * SQUARE_SIZE,
        inner.1 as f32 * SQUARE_SIZE,
        0.,
    )
}

fn handle_input(input: Res<Input<KeyCode>>, mut query: Query<&mut RequestedDirection>) {
    if let Some(new_requested_direction) = Direction::change(input) {
        query.single_mut().0 = new_requested_direction;
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
struct Coordinates((i16, i16));

impl From<(i16, i16)> for Coordinates {
    fn from(value: (i16, i16)) -> Self {
        Coordinates(value)
    }
}

impl From<Coordinates> for (i16, i16) {
    fn from(value: Coordinates) -> Self {
        let inner = value.0;
        (inner.0, inner.1)
    }
}

fn process_position(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    mut tile_query: Query<(&mut Coordinates, &TileType, Entity), With<TileType>>,
    mut segment_order_query: Query<&mut TailOrder>,
    mut head_query: Query<(&mut Direction, &mut Coordinates), Without<TileType>>,
    requested_direction_query: Query<&RequestedDirection>,
    game_query: Query<Entity, With<Game>>,
) {
    let (mut head_direction, mut head_position) = head_query.single_mut();
    let old_head_position = *head_position;

    let requested_direction = requested_direction_query.single().0;
    *head_direction = if requested_direction != head_direction.opposite() {
        requested_direction
    } else {
        *head_direction
    };

    let segments: &mut Vec<Entity> = &mut segment_order_query.single_mut().segments;

    *head_position = head_direction.next_coordinates(old_head_position);
    if let Some((_, tile_type, tile_entity)) = tile_query
        .iter()
        .find(|(tile_coordinates, _, _)| *head_position == **tile_coordinates)
    {
        match tile_type {
            TileType::Reward => {
                commands.entity(tile_entity).despawn();

                commands
                    .entity(game_query.single())
                    .with_children(|commands| {
                        let tail_segment_entity = commands
                            .spawn(TileType::GameOver)
                            .insert(old_head_position)
                            .insert(create_square())
                            .id();
                        segments.push(tail_segment_entity);
                    });
            }
            TileType::GameOver => state.set(GameState::Menu),
        }
    } else {
        for (i, segment_entity) in segments.iter().enumerate() {
            *tile_query
                .get_component_mut::<Coordinates>(*segment_entity)
                .unwrap() = if let Some(next_segment_entity) = segments.get(i + 1) {
                *tile_query
                    .get_component::<Coordinates>(*next_segment_entity)
                    .unwrap()
            } else {
                old_head_position
            };
        }
    }
}

fn sprite_movement(mut transform_query: Query<(&mut Transform, &Coordinates)>) {
    for (mut transform, coordinates) in &mut transform_query {
        transform.translation = translation_from_coordinates(*coordinates);
    }
}

fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    commands.entity(game_query.single()).despawn_recursive();
}
