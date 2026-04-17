---
name: kingdom-geopolitics
description: Especialista en gestión de reinos, diplomacia masiva e IA de hordas para "Rogue-Evolución". Implementa sistemas de "Swarm Intelligence" y ticks diplomáticos de 30 días optimizados para hardware de 2 núcleos. Actívalo con: "implementar lógica de reinos", "crear sistema de invasión de hordas" o "simular colapso de facción".
allowed-tools: [ls, cat, cargo, find]
---

# Spec #4: Kingdom Geopolitics & Swarm AI

## 1. Arquitectura de Simulación Geopolítica
[cite_start]La IA debe separar la simulación de alta fidelidad (cerca del jugador) de la simulación abstracta (mapa global) para proteger el CPU[cite: 93, 118, 119]:

### A. Tick Diplomático Maestro (Abstracción Temporal)
- [cite_start]**Frecuencia**: Ejecutar el sistema de tratados, recursos y declaración de guerra solo cada **30 días de juego**[cite: 104, 105].
- [cite_start]**Lógica**: Evalúa `KingdomState` (recursos, corrupción, orden) para decidir movimientos territoriales en el disco sin cargar todas las entidades a RAM[cite: 106, 113, 123].

### B. Movimiento de Horda (Swarm Intelligence)
- [cite_start]**Algoritmo**: *Steering Behaviors* (Boids) basados en un Líder de Horda[cite: 107].
- **Eficiencia**: Solo el "Líder" calcula rutas mediante **Dijkstra Maps**. [cite_start]Los seguidores (~200 unidades) usan funciones matemáticas simples de alineación y separación, ahorrando un 90% de CPU[cite: 108, 109].
- [cite_start]**Sincronización**: La IA completa solo se activa cuando el jugador entra en la misma "Parasanga" (zona local); de lo contrario, se procesa como un cambio numérico de territorio[cite: 118, 119].

## 2. Gestión de Colapso y Población
- **Transferencia de Posesión**: Al caer un reino, los ciudadanos no se eliminan. [cite_start]Se cambia su `KingdomMember` a roles como `Refugee`, `Slave` o `ExperimentSubject`[cite: 110, 111, 112].
- [cite_start]**Hordas Huérfanas**: Si un líder muere, el sistema debe asignar un sucesor inmediatamente para evitar errores de referencia en el ECS[cite: 120].

## 3. Criterios de Calidad (Pass/Fail)
Antes de entregar el sistema geopolítico, la IA debe validar:
- [ ] [cite_start]**Zero-A* Policy**: ¿Se ha evitado el uso de A* para soldados rasos en favor de Boids/Dijkstra?.
- [ ] **Persistencia Independiente**: ¿Los tratados y datos de reinos se guardan en archivos pequeños fuera del dump de entidades?[cite: 123].
- [ ] **Rayon Balancing**: ¿Se usa el segundo núcleo del Celeron exclusivamente para el cálculo de enjambres?[cite: 95].
- [ ] **Líder de Horda**: ¿Existe un mecanismo de respaldo para evitar hordas huérfanas?[cite: 120].

## 4. Tests Obligatorios
La IA debe generar un test de integración en `src/systems/kingdom/diplomacy.rs` que:
1. Simule un ciclo de 30 días para 5 reinos.
2. Valide que una "Invasión" cree un grupo de entidades siguiendo a un líder.
3. Verifique que la muerte del líder no detenga el movimiento del grupo (asignación de sucesor).
4. Confirme que los datos de tratados persistan tras el colapso de un reino.