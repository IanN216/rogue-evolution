# Spec #16: Settings & Resolution Manager

## 1. Misión
Permitir al usuario ajustar la resolución y el modo de pantalla desde el menú principal para adaptarse al hardware nativo (1366x768).

## 2. Requerimientos Técnicos
- **Persistencia de Opciones**: Crear un archivo `settings.json` o usar `bincode` para guardar la resolución preferida y el estado de fullscreen.
- **Cálculo de Aspect Ratio**: Para 1366x768 con tiles de 8x16:
  - Ancho en tiles: 1366 / 8 ≈ 170
  - Alto en tiles: 768 / 16 = 48
  - Configuración recomendada: 170x48.

## 3. Integración de UI
- **Nuevo Estado**: `RunState::Options` y `MainMenuSelection::Options`.
- **Selector de Resolución**: Implementar un submenú en `src/states/options.rs` con las opciones:
  - "Windowed 80x50"
  - "Fullscreen Native (1366x768)"
  - "Back to Menu"

## 4. Criterios de Aceptación
- El menú de inicio debe incluir la opción "Options".
- Cambiar la resolución debe requerir un reinicio o llamar a `ctx.request_resize()` de bracket-lib.