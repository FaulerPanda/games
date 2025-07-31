use bevy::prelude::*;

use crate::structs::GeneralTimer;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn countdown(time: Res<Time>, mut timer: ResMut<GeneralTimer>) -> bool {
    timer.tick(time.delta()).finished()
}
