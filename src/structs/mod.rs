use ::bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct GeneralTimer(pub Timer);
