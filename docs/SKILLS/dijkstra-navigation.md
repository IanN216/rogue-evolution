# Referencia Técnica: Navegación de Hordas y Hot Path Analysis

Este documento define la implementación avanzada de Dijkstra Maps para navegación masiva y análisis de flujo de nivel, optimizada para minimizar el impacto en el procesador Celeron.

## 1. Métrica de Distancia: Chessboard (Chebyshev)
Para Roguelikes ASCII, no se debe usar la distancia Euclidiana.
- **Implementación Correcta**: La distancia entre dos puntos `(x1, y1)` y `(x2, y2)` debe ser `max(|x1 - x2|, |y1 - y2|)`.
- **Razón**: Esto permite el movimiento diagonal con costo uniforme (1.0), que es el estándar de `bracket-lib` y optimiza el cálculo al evitar raíces cuadradas.

## 2. Re-uso de Mapas (Swarm Optimization)
Para cumplir con la "Ley de Cero Desperdicio":
- **No calcular por entidad**: Si 50 monstruos tienen el mismo objetivo (el Jugador), solo se debe calcular **un único Dijkstra Map** por frame.
- **Gradiente de Descenso**: Las entidades en el sistema `swarm.rs` solo deben leer el valor de sus 8 vecinos en el mapa compartido y moverse al tile con el valor más bajo.

## 3. Análisis de "Caminos Calientes" (Hot Path Analysis)
El Dijkstra Map generado para ir del inicio al fin del nivel contiene información estructural valiosa:
- **Hot Path**: Los tiles con valores bajos en el mapa de navegación principal representan el flujo natural del jugador.
- **Ubicación de Contenido**: 
  - **Items Críticos**: Deben aparecer cerca del Hot Path (umbral de distancia < 10).
  - **Secretos/Bonus**: Deben aparecer en zonas con valores de Dijkstra muy altos (callejones sin salida o áreas remotas).

## 4. Optimización de Memoria (4GB RAM)
- **Flat Array**: El mapa de distancias debe ser un `Vec<f32>` plano del mismo tamaño que el mapa de tiles.
- **Caché de Mapas**: Solo recalcular el mapa si el Jugador se ha movido o si la topología del mapa ha cambiado (ej: destrucción de un muro).

## 5. Criterios de Validación para el Auditor (Spec-13)
El Auditor debe marcar como **FALLO** si:
- Cada miembro de la horda intenta calcular su propio camino de forma independiente.
- El sistema de spawn de items no utiliza los valores del Dijkstra Map para decidir la dificultad de acceso.
- Se detectan llamadas a `f32::sqrt` para cálculos de distancia en el bucle de AI.