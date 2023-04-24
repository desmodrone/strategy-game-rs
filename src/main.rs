use bevy::prelude::*;

fn create_battlefield_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const NUM_COLUMNS: usize = 20;
    const NUM_ROWS: usize = 20;
    const TILE_SIZE: f32 = 16.0;

    let tiles_handle = asset_server.load("Tiles/FullTileset.png");
    let tiles_atlas = TextureAtlas::from_grid(
        tiles_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        NUM_COLUMNS,
        NUM_ROWS,
        None,
        None,
    );

    commands.spawn(Camera2dBundle::default());

    const SCALE: f32 = 4.0;
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tiles_atlas_handle,
        sprite: TextureAtlasSprite::new(3),
        transform: Transform::from_scale(Vec3::splat(SCALE)),
        ..default()
    });
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_battlefield_system)
        .run();
}
