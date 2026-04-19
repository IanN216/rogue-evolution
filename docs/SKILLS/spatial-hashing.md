# Referencia Técnica: Hash Espacial de Alto Rendimiento (Celeron Optimized)

Este documento define la implementación correcta para la detección de proximidad masiva (Infección y AI), eliminando la dependencia de `HashMap` para cumplir con la "Ley de Cero Desperdicio".

## 1. Estructura de Datos: La Rejilla Plana (Flat Grid)
En lugar de un `HashMap<(i32, i32), Vec<Entity>>`, se debe usar un vector pre-asignado que represente el mundo dividido en celdas de tamaño `S`.

- **Cálculo de Celda**: 
  - `cell_x = pos_x / CELL_SIZE`
  - `cell_y = pos_y / CELL_SIZE`
  - `index = cell_y * GRID_WIDTH + cell_x`
- **Almacenamiento**: Un `Vec<Vec<Entity>>` donde el vector externo tiene un tamaño fijo de `(Width/S) * (Height/S)`.

## 2. Optimización de Memoria (4GB RAM)
Para evitar miles de pequeñas asignaciones de memoria (`Vec<Entity>` por cada celda):
1. **Doble Búfer**: Usar un único vector plano de entidades ordenadas por su ID de celda.
2. **Índices de Rango**: Un vector secundario de "punteros" que indique dónde empieza y termina cada celda en el vector principal.
3. **Re-uso de Memoria**: Usar `.clear()` en lugar de crear vectores nuevos en cada tick para no disparar el recolector de basura o fragmentar la RAM.

## 3. Paralelismo con Rayon
Dado que el procesador Celeron tiene 2 núcleos físicos:
- La fase de "Llenado de Rejilla" debe ser secuencial (para evitar bloqueos/mutex).
- La fase de "Consulta de Proximidad" (procesar infección) DEBE usar `.par_iter()` sobre las celdas activas para dividir la carga de trabajo.

## 4. Criterios de Validación para el Auditor (Spec-13)
El Auditor debe marcar como **FALLO** si:
- Se detecta el uso de `std::collections::HashMap` en el sistema de infección o IA.
- El tamaño de la celda no es una potencia de 2 (ej: 4 o 8), ya que el Celeron procesa mucho más rápido las divisiones por desplazamiento de bits (`>>`).
- El sistema recrea la estructura de datos desde cero en cada tick en lugar de limpiar la existente.