# name: map-gen-expert-v2
# description: Finaliza la generación de mapas y valida la conectividad.

## Objetivos Técnicos
1. **Algoritmo**: Implementar un generador de cuevas mediante **Autómatas Celulares** o **Drunkard's Walk** en `src/core/map_gen.rs`.
2. **Criterio de Inundación (Flood Fill)**: Implementar una función que verifique que el 100% de los tiles transitables sean alcanzables desde el punto de inicio del jugador.
3. **Manejo de "Tiles Huérfanos"**: Si un tile de suelo queda aislado por muros, el sistema debe convertirlo automáticamente en muro o abrir un túnel.

## Criterios de Aceptación (100%)
- [ ] El mapa se genera sin áreas cerradas inaccesibles.
- [ ] El código usa `flat vectors` para el mapa (DOD) para ahorrar memoria.
- [ ] No existen comentarios "TODO" en la lógica de generación.
