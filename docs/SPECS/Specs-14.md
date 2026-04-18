# name: map-inspector-tool
# description: Herramienta de depuración para explorar mapas con zoom y detección de errores.

## Funcionalidades
1. **Estado de Inspección**: Crear el estado `RunState::MapInspector` en `src/states/mod.rs`.
2. **Zoom Dinámico**: Usar las teclas `[+]` y `[-]` para cambiar la escala de la consola de `bracket-lib` (ajustando el viewport visible).
3. **Detección de "Mala Colocación"**: 
    - Resaltar en **Rojo** tiles de suelo sin vecinos transitables.
    - Resaltar en **Amarillo** tiles de muro que no tienen grosor (muros flotantes).
4. **Coordenadas**: Mostrar en el HUD la posición `(x, y)` del cursor y el tipo de tile.

## Restricciones Celeron
- No renderizar todo el mapa masivo a la vez; solo lo que cabe en el viewport de la consola virtual.
