# Referencia Técnica: Autómatas Celulares con Límites de Frontera (Antibacktracking)

Este documento define la implementación avanzada de generación de cavernas para "Rogue-Evolution", diseñada para crear niveles largos y estrechos que favorecen la progresión constante.

## 1. Control de Proporción (Narrowness)
Para evitar cámaras excesivamente abiertas donde la IA de hordas se disperse:
- **Técnica de Kyzrati**: En lugar de inicializar el mapa al 50/50 de forma uniforme, se debe aplicar una máscara de probabilidad.
- **Probabilidad Gradual**: La probabilidad de que un tile sea "Muro" debe aumentar a medida que la coordenada `Y` se aleja de la línea central horizontal (o `X` de la vertical).
- **Efecto**: Esto genera cuevas con una "direccionalidad" clara (ej: de Oeste a Este), reduciendo rutas circulares infinitas.

## 2. Manejo de Fronteras (Edge Padding)
Para garantizar que el jugador nunca se sienta "atrapado" contra los bordes:
- **Reserva de Conectividad**: Los bordes cardinales (N, S, E, O) deben ser inicializados como Suelo en puntos específicos antes de las iteraciones de suavizado.
- **Fijación de Muros**: Obligar a que las 2 filas/columnas exteriores sean siempre `Wall` para evitar que el algoritmo de Flood Fill intente conectar con el vacío fuera del mapa.

## 3. Eliminación de Pilares Aislados (Smoothing passes)
Las iteraciones estándar de CA suelen dejar pilares de 1x1 que estorban el movimiento de la horda.
- **Regla 4-5 Refinada**: 
  - Si un tile es Suelo y tiene < 2 vecinos de Suelo, se convierte en Muro (elimina motas).
  - Si un tile es Muro y tiene > 5 vecinos de Suelo, se convierte en Suelo (abre cuellos de botella).

## 4. Validación de Jugabilidad (Spec-1.1)
El mapa no se considera válido solo por ser "orgánico". Debe cumplir:
- **Flood Fill Maestro**: El 100% del espacio de juego debe ser una única región conectada.
- **Métrica de Backtracking**: El camino más corto entre la entrada y la salida debe cubrir al menos el 60% de la extensión total del mapa en el eje dominante.

## 5. Criterios de Validación para el Auditor (Spec-13)
El Auditor debe marcar como **FALLO** si:
- El generador de mapas no aplica un sesgo de probabilidad basado en coordenadas para forzar la forma del nivel.
- Existen "pilares" de muro solitarios (1x1) rodeados totalmente de suelo.
- La función `ensure_connectivity` deja más de una región navegable.