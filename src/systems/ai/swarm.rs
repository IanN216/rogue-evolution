use hecs::World;
use crate::components::stats::Position;
use crate::components::kingdom::{HordeLeader, HordeMember};

pub fn process_swarm_ai(world: &mut World) {
    let mut movements = Vec::new();

    let mut leaders = std::collections::HashMap::new();
    for (entity, (pos, _)) in world.query::<(&Position, &HordeLeader)>().iter() {
        leaders.insert(entity, *pos);
    }

    for (entity, (pos, member)) in world.query::<(&mut Position, &HordeMember)>().iter() {
        if let Some(leader_pos) = leaders.get(&member.leader_entity) {
            let dx = leader_pos.x - pos.x;
            let dy = leader_pos.y - pos.y;

            let dist = ((dx*dx + dy*dy) as f32).sqrt();
            let (mut move_x, mut move_y) = (0, 0);

            if dist > 2.0 {
                move_x = dx.signum();
                move_y = dy.signum();
            } else if dist < 1.5 {
                move_x = -dx.signum();
                move_y = -dy.signum();
            }

            if move_x != 0 || move_y != 0 {
                movements.push((entity, move_x, move_y));
            }
        }
    }

    for (entity, dx, dy) in movements {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.x += dx;
            pos.y += dy;
        }
    }
}
