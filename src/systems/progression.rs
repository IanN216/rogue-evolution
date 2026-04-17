use hecs::World;
use crate::components::progression::{Experience, AbilityRegistry, Humanoid, Ability};
use crate::components::stats::BaseStats;

pub fn process_progression(world: &mut World) {
    let mut leveled_up = Vec::new();

    for (entity, (exp, stats)) in world.query_mut::<(&mut Experience, &mut BaseStats)>() {
        if exp.xp >= exp.next_level_xp {
            exp.level += 1;
            exp.xp -= exp.next_level_xp;
            exp.next_level_xp = exp.level * 100; // Escalado simple para el demo

            // Mejora lineal de stats por nivel
            stats.max_hp += 5;
            stats.hp = stats.max_hp;
            stats.attack += 1;
            stats.defense += 1;

            leveled_up.push((entity, exp.level));
        }
    }

    for (entity, level) in leveled_up {
        if level % 20 == 0 {
            // Solo humanoides reciben habilidades de clase cada 20 niveles
            let is_humanoid = world.get::<&Humanoid>(entity).is_ok();
            
            if is_humanoid {
                let new_ability = Ability {
                    name: format!("Class Mastery Tier {}", level / 20),
                    hp_bonus: 20,
                    attack_bonus: 5,
                    defense_bonus: 5,
                };

                // Aplicar bonus permanentemente a BaseStats
                if let Ok(mut stats) = world.get::<&mut BaseStats>(entity) {
                    stats.max_hp += new_ability.hp_bonus;
                    stats.hp = stats.max_hp;
                    stats.attack += new_ability.attack_bonus;
                    stats.defense += new_ability.defense_bonus;
                }

                // Intentar obtener el AbilityRegistry existente
                let mut registry_opt = None;
                if let Ok(mut registry) = world.get::<&mut AbilityRegistry>(entity) {
                    registry.abilities.push(new_ability.clone());
                } else {
                    // Si no existe, lo crearemos fuera del scope del borrow
                    let mut new_registry = AbilityRegistry::new();
                    new_registry.abilities.push(new_ability);
                    registry_opt = Some(new_registry);
                }

                if let Some(registry) = registry_opt {
                    world.insert_one(entity, registry).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::stats::BaseStats;

    #[test]
    fn test_humanoid_progression_at_level_20() {
        let mut world = World::new();
        
        let entity = world.spawn((
            Humanoid,
            Experience { level: 19, xp: 1900, next_level_xp: 1900 },
            BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 10 },
            AbilityRegistry::new(),
        ));

        process_progression(&mut world);

        let exp = world.get::<&Experience>(entity).unwrap();
        assert_eq!(exp.level, 20);

        let registry = world.get::<&AbilityRegistry>(entity).unwrap();
        assert_eq!(registry.abilities.len(), 1);
        assert_eq!(registry.abilities[0].name, "Class Mastery Tier 1");

        let stats = world.get::<&BaseStats>(entity).unwrap();
        // 100 base + 5 (level up) + 20 (ability) = 125
        assert_eq!(stats.max_hp, 125);
    }

    #[test]
    fn test_monster_no_class_ability_at_level_20() {
        let mut world = World::new();
        
        // Entidad SIN Humanoid
        let entity = world.spawn((
            Experience { level: 19, xp: 1900, next_level_xp: 1900 },
            BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 10 },
        ));

        process_progression(&mut world);

        let exp = world.get::<&Experience>(entity).unwrap();
        assert_eq!(exp.level, 20);

        // No debería tener AbilityRegistry porque no es Humanoid
        assert!(world.get::<&AbilityRegistry>(entity).is_err());
        
        let stats = world.get::<&BaseStats>(entity).unwrap();
        // 100 base + 5 (level up) = 105 (sin bonus de habilidad)
        assert_eq!(stats.max_hp, 105);
    }
}
