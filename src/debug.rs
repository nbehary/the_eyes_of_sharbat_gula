use bevy::prelude::*;

pub struct DebugPlugin;

use crate::movement::Velocity;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Velocity)>){
    for (entity, velocity) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, velocity);
    }

}
