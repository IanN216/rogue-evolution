# Spec: Reglas de Oro de la Interfaz (UI-UX)

Este documento define el estándar obligatorio para la creación de interfaces en Rogue-Evolution, optimizado para el Celeron N2806.

## 1. Centrado Dinámico Absoluto
Prohibido el uso de coordenadas hardcodeadas (ej. 25, 40). Todo elemento debe posicionarse relativo al tamaño de la consola activa.
- **Centro X**: `let (sw, _) = ctx.get_char_size(); let x = (sw as i32 / 2) - (box_width / 2);`
- **Centro Y**: `let (_, sh) = ctx.get_char_size(); let y = (sh as i32 / 2) - (box_height / 2);`

## 2. Protocolo de Limpieza Total (Anti-Ghosting)
Para evitar que restos de menús anteriores se superpongan al gameplay o a nuevos estados:
- **Limpieza Triple**: Al entrar en cualquier `tick` de estado que use UI, se deben limpiar las 3 consolas (0, 1, 2) mediante `draw_batch.cls()` o `ctx.cls()`.
- **Targeting Explícito**: Siempre declarar `draw_batch.target(2)` para UI para asegurar que el buffer se limpie en la capa correcta.

## 3. Construcción de Recuadros (Boxes)
- **Capa UI (2)**: Los recuadros nunca deben dibujarse en la capa 0 (Mapa) o 1 (Entidades).
- **Z-Order**: El gameplay siempre ocurre debajo; la UI siempre arriba. Al cambiar de estado de `InGame` a `MainMenu`, la capa 0 y 1 DEBEN ser vaciadas.

## 4. Eficiencia Celeron
- **DrawBatch**: Preferir `DrawBatch` sobre llamadas directas a `ctx` para minimizar los cambios de contexto en la GPU.
- **Submit Único**: Realizar un único `submit(0)` al final del tick del estado.
