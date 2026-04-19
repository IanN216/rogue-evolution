use hecs::World;
use crate::components::stats::Position;
use crate::components::items::{Item, Blighted, InfectionSource};
use crate::components::genetics::Genetics;

pub fn process_item_infection(world: &mut World) {
    let mut to_blight = Vec::new();
    let mut infection_sources = Vec::new();

    // 1. Gather all infection sources positions
    for (_, (pos, _)) in world.query::<(&Position, &InfectionSource)>().iter() {
        infection_sources.push((pos.x, pos.y));
    }

    // 2. Infect items in the same position
    for (entity, (pos, _item)) in world.query::<(&Position, &Item)>().without::<&Blighted>().iter() {
        if infection_sources.iter().any(|(ix, iy)| *ix == pos.x && *iy == pos.y) {
            to_blight.push(entity);
        }
    }

    for entity in to_blight {
        world.insert_one(entity, Blighted).unwrap();
    }

    // 3. Infect entities holding or near blighted items
    let mut exposure_increase = Vec::new();
    let mut blighted_item_positions = Vec::new();

    for (_, (pos, _)) in world.query::<(&Position, &Blighted)>().iter() {
        blighted_item_positions.push((pos.x, pos.y));
    }

    for (entity, (pos, _genetics)) in world.query::<(&Position, &mut Genetics)>().iter() {
        if blighted_item_positions.iter().any(|(bx, by)| *bx == pos.x && *by == pos.y) {
            exposure_increase.push(entity);
        }
    }

    for entity in exposure_increase {
        if let Ok(mut genetics) = world.get::<&mut Genetics>(entity) {
            genetics.exposure_level += 0.5; // Small exposure increase per tick
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::stats::Position;
    use crate::components::items::{Item, MaterialType, InfectionSource, Blighted, Weapon};
    use crate::components::genetics::Genetics;

    #[test]
    fn test_stat_inheritance() {
        let iron_material = MaterialType::Iron;
        let glass_material = MaterialType::Glass;
        let sword = Weapon { base_damage: 10 };

        assert_eq!(sword.get_damage(&iron_material), 10);
        assert_eq!(sword.get_damage(&glass_material), 15);
    }

    #[test]
    fn test_item_infection() {
        let mut world = World::new();

        // Infection source at (5,5)
        world.spawn((Position { x: 5, y: 5 }, InfectionSource));

        // Clean item at (5,5)
        let item_e = world.spawn((
            Position { x: 5, y: 5 },
            Item { name: "Rusty Dagger".to_string(), material: MaterialType::Iron }
        ));

        process_item_infection(&mut world);

        // Item should now be blighted
        assert!(world.get::<&Blighted>(item_e).is_ok());
    }

    #[test]
    fn test_blight_infects_bearer() {
        let mut world = World::new();

        // Blighted item at (10,10)
        world.spawn((Position { x: 10, y: 10 }, Item { name: "Cursed Orb".to_string(), material: MaterialType::Organic }, Blighted));

        // Healthy entity at (10,10)
        let entity = world.spawn((
            Position { x: 10, y: 10 },
            Genetics::new()
        ));

        process_item_infection(&mut world);

        let genetics = world.get::<&Genetics>(entity).unwrap();
        assert!(genetics.exposure_level > 0.0);
    }
}
