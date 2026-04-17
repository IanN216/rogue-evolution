---
name: rogue-evolution-master-context
description: Contexto maestro para el proyecto "Ecosistema Rogue-Evolución". Define las restricciones de hardware y el inventario de Agent Skills (Specs) para garantizar un desarrollo eficiente en Rust.
---

# GEMINI.md - Ecosistema Rogue-Evolución

## 1. Visión del Proyecto
[cite_start]**Rogue-Evolución** es un simulador geopolítico y biológico tipo Roguelike desarrollado en **Rust**[cite: 1, 3]. [cite_start]Está diseñado para ofrecer una profundidad de simulación masiva (estilo *Cave of Qud* o *Elona*) funcionando en hardware de recursos críticos[cite: 2].

### 🛠️ Restricciones de Hardware (Target)
- [cite_start]**CPU**: Intel Celeron N2806 (2 núcleos, 1.60 GHz)[cite: 2].
- [cite_start]**RAM**: 4GB DDR3 (Límite operativo: 1GB para la simulación activa)[cite: 2, 45].
- [cite_start]**Prioridad**: Eficiencia extrema en ciclos de CPU y optimización del caché L2[cite: 2, 4, 31, 36].

---

## 2. Inventario de Agent Skills (Specs)
[cite_start]Este proyecto utiliza una arquitectura de **Skills** para minimizar el uso de tokens y evitar alucinaciones[cite: 155, 281]. Los Specs se encuentran en: `rogue-evolution/docs/SPECS/`.

| Archivo | Skill Name | Propósito y Momento de Uso |
| :--- | :--- | :--- |
| **Spec-0.md** | `project-setup` | [cite_start]Configuración de Cargo.toml, compilador y auditoría de carpetas. [cite: 288] |
| **Spec-1.md** | `map-gen-expert` | Implementación de Drunkard's Walk y Autómatas Celulares. |
| **Spec-2.md** | `ecs-optimizer` | Diseño de componentes planos y optimización de caché (Hot/Cold). |
| **Spec-3.md** | `biological-engine`| Genética, herencia mendeliana e infección por proximidad. |
| **Spec-4.md** | `kingdom-geopolitics`| Gestión de reinos, diplomacia masiva y Swarm AI (Boids). |

---

## 3. Reglas de Oro (Leyes del Proyecto)
Cualquier código generado debe obedecer estas restricciones para evitar el colapso del Celeron:

1. **Zero-A* Policy**: Prohibido usar A* para hordas o entidades de baja inteligencia. Usar **Dijkstra Maps** o **Boids**[cite: 31, 107, 108].
2. **Data-Oriented Design (DOD)**: Priorizar el uso de **Flat Vectors** sobre estructuras anidadas o `HashMap`[cite: 36, 40].
3. **División Hot/Cold**: Los datos de movimiento deben estar físicamente separados de los datos de historial/genealogía en el ECS[cite: 40, 75].
4. **Batching Paralelo**: Las tareas de `rayon` deben procesarse en lotes (chunks) de **500 entidades** para no saturar los 2 núcleos con el overhead de hilos[cite: 86, 94].
5. **Abstracción Temporal**: Los sistemas pesados (diplomacia, infección) deben ejecutarse cada X ticks, nunca cada frame[cite: 82, 105].

---

## 4. Protocolo de Carga para la IA
1. **Nivel 1 (Metadata)**: Al iniciar, lee este `GEMINI.md` para conocer las restricciones[cite: 153].
2. **Nivel 2 (Instrucciones)**: Cuando la tarea coincida con un trigger, carga el Spec correspondiente desde `docs/SPECS/`[cite: 156].
3. **Nivel 3 (Referencias)**: Si el Spec lo requiere, solicita archivos de referencia específicos (ej. `Cargo.toml` o `main.rs`)[cite: 158].

---
**Allowed Tools**: [ls, cat, cargo, mkdir, find, rm]