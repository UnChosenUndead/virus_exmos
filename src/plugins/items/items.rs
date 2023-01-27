use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub enum ItemType {
    Weapon,
    Armor,
    Amulet,
    Ring,
    ThrowableItems,
    Flask,
    Ammunition,
    Other,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Item;

fn drop_item() {}

fn take_item() {}

fn equip_item() {}
