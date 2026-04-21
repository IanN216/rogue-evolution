# GEMINI.md - Rogue-Evolution: Motor de Generación Planetaria

## 1. Misión
Creación de un biosistema planetario procedural sin costuras. El objetivo actual es construir un mundo de $512 \times 512$ tiles estructurado en regiones, con topología puramente toroidal, donde la geografía fluye perfectamente a través de los bordes.

## 2. Estrategia Técnica
Priorizar la **Plenitud Algorítmica**, la coherencia visual y topológica por encima de la micro-optimización de ciclos de CPU. El Celeron N2806 es capaz de manejar grandes estructuras dinámicas si se utiliza paralelización controlada (Rayon).

## 3. Estado del Proyecto
**Fase 1 - Motor de Generación Planetaria:**
* Implementación de coordenadas modulares absolutas (`rem_euclid`).
* Ruido Simplex/OpenSimplex para elevación y biomas usando coordenadas globales.
* Autómatas celulares y Drunkard's Walk toroidales que atraviesan los bordes del mapa.
* Visualizador interactivo de cámara libre en pantalla completa.

## 4. Reglas Estrictas de Generación
* **Zero-Walls Border Policy**: Bajo ningún concepto se usarán muros artificiales para enmarcar el mapa. El mundo debe ser una esfera sin fronteras perceptibles.
* **Global Continuity**: Todas las semillas de ruido deben muestrear sobre coordenadas globales normalizadas.
* **No ECS overhead**: Durante la Fase 1, se prescindirá de cualquier lógica de entidades para enfocarse 100% en la topología matemática.