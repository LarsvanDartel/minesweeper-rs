use bevy::prelude::*;

pub fn despawn_all<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive()
    }
}
