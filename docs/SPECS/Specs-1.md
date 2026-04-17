---
name: map-gen-expert
description: Especialista en generación procedimental de mapas orgánicos para "Rogue-Evolución". Implementa Drunkard's Walk y Autómatas Celulares optimizados para baja RAM. Actívalo con: "generar mapa de cuevas", "implementar algoritmo de caminante borracho" o "optimizar generación procedimental".
allowed-tools: [ls, cat, cargo, find]
---

# Spec #1: Map Generation & Connectivity Validation

## 1. Algoritmos Requeridos
La IA debe priorizar el uso de memoria constante sobre la elegancia visual:
- **Drunkard's Walk**: Para túneles. [cite_start]No debe usar estructuras de datos auxiliares pesadas.
- **Cellular Automata**: Para cavernas. [cite_start]Debe usar `rayon` para procesar la rejilla en paralelo aprovechando los 2 núcleos del Celeron[cite: 55, 95].

## 2. Restricciones Técnicas
- **Flat Vectors**: El mapa DEBE ser un `Vec<TileType>` plano. [cite_start]Prohibido el uso de `Vec<Vec<T>>`[cite: 32, 40].
- [cite_start]**Indexing**: Uso de la fórmula `(y * width) + x` para acceso directo a memoria[cite: 33].

## 3. Criterios de Calidad (Pass/Fail)
Antes de entregar el código, la IA debe verificar:
- [ ] [cite_start]**Conectividad**: ¿Se ha implementado un test (ej. Flood Fill) que asegure que el 100% de los suelos son accesibles? 
- [ ] [cite_start]**Zero-Allocation**: ¿El algoritmo de generación evita crear copias innecesarias del mapa en RAM? [cite: 45]
- [ ] [cite_start]**Deterministic Seeds**: ¿Se utiliza una semilla (`seed`) para que la generación sea reproducible? [cite: 56]
- [ ] [cite_start]**Performance**: ¿La generación de un mapa de 80x50 toma menos de 100ms en el hardware objetivo? [cite: 82]

## 4. Test Obligatorio
La IA debe generar un test unitario en `src/core/map.rs` que:
1. Genere un mapa completo.
2. Valide que no existan "islas" de suelo inaccesibles.
3. Verifique que el número de muros no supere el 60% del área total para garantizar espacio de movimiento.