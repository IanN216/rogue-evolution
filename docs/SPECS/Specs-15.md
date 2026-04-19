# Spec #15: Display Architect & Fullscreen Manager

## 1. Misión
Implementar el modo pantalla completa optimizado para la Intel HD Graphics del Celeron N2806, asegurando que el escalado ASCII no genere latencia de refresco.

## 2. Implementación Técnica (BTermBuilder)
En `src/main.rs`, se debe modificar la inicialización del contexto:

- **Modo Pantalla Completa**: Usar el método `.with_fullscreen(true)` del `BTermBuilder`.
- **Escalado de Resolución**: Para el Celeron, se debe usar `.with_advanced_input(true)` para permitir que la ventana maneje cambios de tamaño sin colapsar el ECS.
- **Optimización de Ticks**: Asegurar que `with_fps_cap(60.0)` esté activo para evitar que el CPU intente renderizar frames innecesarios en segundo plano.

## 3. Comandos de Usuario
- **Atajo Global**: Implementar `Alt + Enter` o `F11` dentro de `src/main.rs` o `states/ingame.rs` para alternar entre modo ventana y pantalla completa usando `ctx.with_fullscreen(bool)`.

## 4. Criterios de Aceptación (Auditoría)
El Auditor (Spec-13) marcará como **SISTEMA 100% OPERATIVO** solo si:
- El juego inicia en pantalla completa por defecto (o según configuración).
- No hay caídas de FPS detectadas en el log al cambiar de modo.
- El escalado de los tiles ASCII se mantiene proporcional a la relación de aspecto 16:9/16:10 de la netbook.