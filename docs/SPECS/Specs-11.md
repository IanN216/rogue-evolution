name: regional-map-expert
description: Especialista en la implementación técnica de la generación de Parasangas. Usa Drunkard's Walk y validación de conectividad regional. Actívalo con: "implementar generador de Parasangas", "programar algoritmo de caminante borracho regional" o "conectar zonas del mapa mundial".
allowed-tools: [ls, cat, cargo]
Spec #11: Regional Map Generation (Drunkard's Walk)
1. Algoritmos y Lógica de Construcción

Basado en el Spec #1, este Spec detalla la implementación en src/core/map_gen.rs:

    Drunkard's Walk: Para cavar túneles y cavernas orgánicas.

        Lifetime: El "caminante" debe tener un límite de pasos para evitar limpiar todo el mapa.

        Coverage: La generación se detiene cuando el suelo alcanza el 40-50% del área local.

    Connectivity: Implementar un sistema de "puntos de salida" (bordes del mapa) que asegure que el jugador pueda transitar entre Parasangas adyacentes.

2. Restricciones Técnicas

    RNG: Uso obligatorio de bracket_lib::prelude::RandomNumberGenerator con semillas (seed) persistentes para reproducibilidad.

    Zero-Allocation: No crear mapas temporales; trabajar directamente sobre el Vec<TileType> de la región actual.

3. Test de Validación

    Generar 10 mapas seguidos con la misma semilla.

    Validar que los 10 archivos binarios resultantes sean idénticos (Integridad del RNG).

Spec #12: Dijkstra Navigation & Pathfinding
name: pathfinding-specialist
description: Experto en navegación basada en mapas de Dijkstra para hordas y entidades inteligentes. Reemplaza el movimiento lineal por navegación consciente de obstáculos. Actívalo con: "implementar Dijkstra Map", "optimizar navegación de hordas" o "crear mapa de costos de movimiento".
allowed-tools: [ls, cat, cargo]