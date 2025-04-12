use std::sync::{Arc, OnceLock};

use bevy::prelude::*;

use assets::plant_data::PlantTypeId;
use common::traits::AddNamedObserver;

mod plant;
pub use plant::Plant;
mod grow_system;
pub use grow_system::grow_system;
mod on_add_carrot;
pub use on_add_carrot::on_add_carrot;

pub fn carrot_id() -> PlantTypeId {
    static CARROT: OnceLock<Arc<str>> = OnceLock::new();
    CARROT
        .get_or_init(|| Arc::from("Carrot"))
        .clone()
}


pub type Number = i32;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, grow_system);
    app.add_named_observer(on_add_carrot, "on_add_carrot");
}








#[cfg(test)]
mod tests {
/*
    use super::*;
    #[test]
    fn it_works() {
        let result = 2  + 2;
        assert_eq!(result, 4);
    }
*/
}
