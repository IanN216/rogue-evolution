use bracket_lib::prelude::*;
use super::RunState;
use crate::core::world::WorldManager;
use crate::core::map_gen::{generate_caverns_step, drunkard_walk_step, add_regional_exits, ensure_connectivity_step};
use crate::components::stats::{Position, Renderable, Viewshed, BaseStats};
use crate::components::identity::Identity;
use crate::components::progression::Experience;

pub fn tick(ctx: &mut BTerm, wm: &mut WorldManager, phase: usize, progress: f32, phase_step: usize) -> Option<RunState> {
    // 1. Método 'Master Clear' (Eliminar Fantasmas Visuales)
    for i in 0..2 {
        ctx.set_active_console(i);
        ctx.cls();
    }

    // 2. Implementación de Geometría Dinámica
    let (sw, sh) = ctx.get_char_size();
    
    let bw = 64; // Ancho de la caja
    let bh = 10; // Alto de la caja
    let x = (sw as i32 / 2) - (bw / 2);
    let y = (sh as i32 / 2) - (bh / 2);
    
    // Capa 0: Fondo (Opcional, cls() ya limpia)
    ctx.set_active_console(0);
    
    // Capa 1: UI
    ctx.set_active_console(1);
    ctx.draw_hollow_box(x, y, bw, bh, RGB::named(WHITE), RGB::named(BLACK));
    
    let phase_text = match phase {
        0 => "Fase 1: Autómatas Celulares (Estructura Inicial)",
        1 => "Fase 2: Drunkard's Walk (Erosión y Cavidades)",
        2 => "Fase 3: Sincronización y Conectividad",
        3 => "Fase 4: Spawning de Entidades Bio-Sintéticas",
        _ => "Generación Completada - Sistema Estable",
    };

    ctx.print_color_centered(y + 2, RGB::named(YELLOW), RGB::named(BLACK), "GENERANDO MUNDO PROCEDURAL");
    ctx.print_centered(y + 4, phase_text);
    
    // Barra de Progreso centrada
    let bar_w = 50;
    let bar_x = (sw as i32 / 2) - (bar_w / 2);
    let total_progress = (phase as f32 + progress) / 4.0;
    
    ctx.draw_bar_horizontal(bar_x, y + 6, bar_w, (total_progress * 100.0) as i32, 100, RGB::named(CYAN), RGB::named(BLACK));
    ctx.print_centered(y + 8, format!("{:.0}%", total_progress * 100.0));

    // Lógica de generación por fases (No bloqueante)
    match phase {
        0 => {
            if phase_step < 10 {
                let new_progress = generate_caverns_step(&mut wm.world_map.map, phase_step, 8888);
                return Some(RunState::MapGen { phase: 0, progress: new_progress, phase_step: phase_step + 1 });
            } else {
                return Some(RunState::MapGen { phase: 1, progress: 0.0, phase_step: 0 });
            }
        }
        1 => {
            if progress < 1.0 {
                let new_progress = drunkard_walk_step(&mut wm.world_map.map, phase_step, 8888);
                return Some(RunState::MapGen { phase: 1, progress: new_progress, phase_step: phase_step + 1 });
            } else {
                add_regional_exits(&mut wm.world_map.map);
                return Some(RunState::MapGen { phase: 2, progress: 0.0, phase_step: 0 });
            }
        }
        2 => {
            ensure_connectivity_step(&mut wm.world_map.map);
            return Some(RunState::MapGen { phase: 3, progress: 0.0, phase_step: 0 });
        }
        3 => {
            spawn_player(wm, sw as i32, sh as i32);
            return Some(RunState::MapGen { phase: 4, progress: 1.0, phase_step: 0 });
        }
        4 => {
            ctx.print_centered(y + bh + 2, "MAPA LISTO. PRESIONE [ESPACIO]");
            if let Some(VirtualKeyCode::Space) = ctx.key {
                return Some(RunState::InGame);
            }
        }
        _ => {}
    }

    None
}

fn spawn_player(wm: &mut WorldManager, width: i32, height: i32) {
    let mut exists = false;
    for (_entity, id) in wm.world.query::<&Identity>().iter() {
        if id.name == "Hero" { exists = true; break; }
    }

    if !exists {
        let player_start = Position { x: width / 2, y: height / 2 }; 
        wm.world.spawn((
            player_start,
            Renderable { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) },
            Viewshed { visible_tiles: Vec::new(), range: 12, dirty: true },
            Identity { name: "Hero".to_string(), title: None, kingdom_id: 0 },
            BaseStats { hp: 100, max_hp: 100, attack: 10, defense: 5 },
            Experience::new(),
        ));
    }
}
