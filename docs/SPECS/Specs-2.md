---
name: ecs-optimizer
description: Experto en arquitectura ECS con hecs y optimización Data-Oriented para "Rogue-Evolución". Diseña componentes planos, audita sistemas de Rayon y optimiza el acceso al caché del CPU. Actívalo con: "diseñar componente hecs", "optimizar sistema paralelo" o "auditar rendimiento de consultas ECS".
allowed-tools: [ls, cat, cargo, find]
---

# Spec #2: ECS Optimizer & Cache Strategist

## 1. Diseño de Componentes (Zero-Bloat)
[cite_start]La IA debe aplicar el principio de "Bolsas de Datos" para maximizar la localidad de referencia[cite: 36, 75]. Los componentes deben ser estructuras planas (Copy/Clone) para evitar saltos en la memoria RAM:

- [cite_start]**Hot Components** (Posición, Velocidad, Stats): Deben ser lo más pequeños posible (usar `f32`, `i32`, `u8`)[cite: 40].
- [cite_start]**Cold Components** (Identidad, Genealogía): Deben estar en arquetipos separados para no "ensuciar" el caché durante el bucle de movimiento.

## 2. Optimización de Sistemas y Rayon
Para el hardware de 2 núcleos, se deben seguir estas reglas de procesamiento masivo:
- [cite_start]**Batching**: El procesamiento debe agruparse en bloques de **500 entidades** por cada "job" de Rayon para minimizar el overhead de gestión de hilos.
- [cite_start]**Archetype Stability**: Prohibido añadir o quitar componentes a entidades dentro del bucle principal (usar "Flags" o booleanos en su lugar) para evitar la fragmentación de memoria en `hecs`.

## 3. Criterios de Calidad (Pass/Fail)
Antes de entregar cualquier sistema o componente, la IA debe validar:

- [ ] **Data Alignment**: ¿Los componentes "Hot" carecen de tipos dinámicos como `String` o `Vec`?
- [ ] [cite_start]**Archetype Check**: ¿El diseño evita que la creación de una entidad genere un arquetipo único innecesario?.
- [ ] [cite_start]**Rayon Overhead**: ¿Los sistemas paralelos procesan entidades en lotes (chunks) en lugar de uno por uno?.
- [ ] [cite_start]**Hot/Cold Split**: ¿Se ha verificado que los datos biográficos no se cargan en la misma consulta que los datos de colisión?[cite: 40].

## 4. Test de Rendimiento de Consultas
La IA debe generar un test en `src/systems/mod.rs` (o un archivo dedicado) que:
1. Spawnee 10,000 entidades con el arquetipo estándar de "monstruo".
2. Ejecute una consulta (`Query`) de movimiento básica.
3. Valide que el tiempo de travesía total sea inferior a **2ms** en el hardware objetivo.