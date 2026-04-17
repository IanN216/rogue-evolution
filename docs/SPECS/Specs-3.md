---
name: biological-engine
description: Especialista en simulación biológica, genética de poblaciones y mecánica de infección para "Rogue-Evolución". Implementa herencia mendeliana, mutaciones por exposición y sistemas de contagio masivo optimizados. Actívalo con: "implementar genética de reproducción", "crear sistema de infección por proximidad" o "gestionar evolución por anomalías".
allowed-tools: [ls, cat, cargo, find]
---

# Spec #3: Biological Engine & Infection Genetics

## 1. Motores Biológicos Requeridos
La IA debe implementar la lógica de vida utilizando procesamiento paralelo y temporalmente diferido:

### A. Herencia y Reproducción (Mendelismo)
- [cite_start]**Algoritmo**: Combinación de vectores de rasgos (`dna`) con factor de mutación aleatorio[cite: 65].
- [cite_start]**Propósito**: Selección natural de estadísticas (velocidad, color, percepción)[cite: 66, 67].

### B. Sistema de Infección (Spatial Hashing)
- [cite_start]**Algoritmo**: En lugar de comparar todas las entidades ($O(n^2)$), se utiliza una rejilla espacial para verificar proximidad[cite: 69, 70].
- [cite_start]**Optimización**: Reduce el uso de CPU en un 90%[cite: 71].
- [cite_start]**Regla de Oro**: La infección se procesa cada **30 o 60 ticks**, nunca en cada frame.

### C. Evolución por Exposición (Threshold Checking)
- [cite_start]**Proceso**: Monitoreo del `exposure_level` a anomalías o cadáveres infectados (`InfectionSource`)[cite: 73, 80].
- [cite_start]**Acción**: Al superar el umbral, se dispara un cambio de arquetipo en `hecs`, lo cual es más eficiente que múltiples condicionales en la IA[cite: 74].

## 2. Restricciones de Memoria y Hilos
- [cite_start]**Batching con Rayon**: El metabolismo y la genética deben procesarse en lotes de **500 entidades** por tarea para optimizar los 2 núcleos del Celeron[cite: 55, 86].
- [cite_start]**Vectores Pequeños**: Las listas de habilidades (`race_abilities`) deben ser compactas para no saturar los 4GB de RAM[cite: 83, 84].

## 3. Criterios de Calidad (Pass/Fail)
Antes de entregar el sistema biológico, la IA debe validar:
- [ ] [cite_start]**Abstracción Temporal**: ¿El sistema de infección respeta el intervalo de ticks configurado? 
- [ ] [cite_start]**Spatial Hashing**: ¿Se evita el cálculo de proximidad global en favor de la rejilla? [cite: 69]
- [ ] [cite_start]**Archetype Mutation**: ¿La evolución cambia el arquetipo de `hecs` en lugar de añadir componentes individuales? [cite: 74]
- [ ] [cite_start]**Memory Safety**: ¿Los vectores de ADN carecen de redundancia y datos pesados? [cite: 76]

## 4. Tests Obligatorios
La IA debe generar un test en `src/systems/biology/reproduction.rs` que:
1. Simule la reproducción de dos entidades con estadísticas opuestas.
2. [cite_start]Valide que el ADN resultante esté dentro del rango parental con la mutación aplicada[cite: 65, 88].
3. [cite_start]Verifique que un cambio de arquetipo tras exposición a anomalías no corrompa la entidad en el ECS[cite: 74].