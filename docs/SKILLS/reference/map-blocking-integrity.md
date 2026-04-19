# Referencia Técnica: Gestión Eficiente de Bloqueos y Contenido (DOD)

Este documento define la implementación optimizada para el procesador Celeron de la lógica de colisión y ocupación de tiles, basada en patrones de alto rendimiento de "Caves of Qud" y "Dwarf Fortress".

## 1. Unificación de Llamadas (Ley de Cero Desperdicio)
Actualmente, el sistema incurre en redundancia al tener `populate_blocked` y `clear_content_index` como funciones separadas que recorren el mismo vector.

- **Implementación Correcta**: Debe existir una única función maestra, `update_map_metadata`, que realice una sola pasada sobre el vector de tiles.
- **Acción**: Primero, inicializar el vector `blocked` basándose únicamente en la topología estática (muros vs. suelo).

## 2. Integración con el ECS (Dynamic Blocking)
El mapa no debe ser una entidad aislada; debe reaccionar a las entidades que poseen el componente `BlocksTile`.

1. **Fase de Limpieza**: Al inicio de cada tick, se resetea el vector `blocked` al estado de los muros.
2. **Fase de Actualización**: Se itera sobre todas las entidades con los componentes `Position` y `BlocksTile`.
3. **Sincronización**: Se marca el índice correspondiente en el vector `blocked` del mapa como `true`.

## 3. Optimización para Celeron (Cache L1/L2 Friendly)
Para evitar la sobrecarga de memoria en 4GB de RAM:
- **Bitsets**: En lugar de un `Vec<bool>`, se recomienda el uso de un bitset o un vector de `u8` donde cada bit represente un estado (bloqueado, visible, explorado).
- **Evitar HashMaps**: No utilizar `HashMap` para limpiar el mapa en cada tick. Los vectores indexados por el ID del tile son $O(1)$ y mucho más rápidos para el caché del CPU.

## 4. Criterios de Validación para el Auditor (Spec-13)
El Auditor debe marcar como **FALLO** si:
- Las funciones `populate_blocked` y `clear_content_index` siguen existiendo de forma independiente y redundante.
- El vector `blocked` no se actualiza con la posición de los monstruos (permitiendo que se encimen).
- Se detectan bucles anidados $O(n^2)$ para verificar colisiones entre entidades en lugar de consultar el vector del mapa.