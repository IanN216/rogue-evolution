use bracket_lib::prelude::*;
use crate::core::map::Map;

/// Calcula el FOV toroidal. Dado que bracket-lib no soporta mapas que se envuelven,
/// simulamos el efecto calculando el FOV desde el origen real y sus proyecciones 
/// en el plano toroidal si el rango alcanza los bordes.
pub fn compute_fov(origin: Point, range: i32, map: &Map) -> Vec<Point> {
    let mut visible_points = std::collections::HashSet::new();

    // Calculamos el FOV para el origen y sus 8 vecinos toroidales
    // (Esto cubre los casos donde el Viewshed cruza las costuras del planeta)
    let offsets = [
        (0, 0),
        (map.width, 0), (-map.width, 0),
        (0, map.height), (0, -map.height),
        (map.width, map.height), (-map.width, -map.height),
        (map.width, -map.height), (-map.width, map.height)
    ];

    for (dx, dy) in offsets {
        let test_origin = Point::new(origin.x + dx, origin.y + dy);
        
        // Solo procesamos si el test_origin está razonablemente cerca de los límites del mapa
        // para no sobrecargar el Celeron innecesariamente.
        if (test_origin.x >= -range && test_origin.x < map.width + range) &&
           (test_origin.y >= -range && test_origin.y < map.height + range) {
            
            let fov = field_of_view(test_origin, range, map);
            for p in fov {
                // Envolvemos cada punto visible resultante
                let wrapped_p = Point::new(
                    p.x.rem_euclid(map.width),
                    p.y.rem_euclid(map.height)
                );
                visible_points.insert(wrapped_p);
            }
        }
    }

    visible_points.into_iter().collect()
}
