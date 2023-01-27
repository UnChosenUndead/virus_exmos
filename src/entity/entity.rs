use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub(crate) value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Manna {
    pub(crate) value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Damage {
    pub(crate) value: i32,
}
