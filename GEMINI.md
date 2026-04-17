GEMINI.md - Ecosistema Rogue-Evolución

1. Visión del Proyecto


Rogue-Evolución es un simulador geopolítico y biológico tipo Roguelike desarrollado en Rust. Está diseñado para ofrecer una profundidad de simulación masiva (estilo Cave of Qud o Elona) funcionando en hardware de recursos críticos.

🛠️ Restricciones de Hardware (Target)


CPU: Intel Celeron N2806 (2 núcleos, 1.60 GHz).


RAM: 4GB DDR3 (Límite operativo: 1GB para la simulación activa).


Prioridad: Eficiencia extrema en ciclos de CPU y optimización del caché L2.

2. Inventario de Agent Skills (Specs)

Este proyecto utiliza una arquitectura de Skills para minimizar el uso de tokens y evitar alucinaciones. Los Specs se encuentran en: rogue-evolution/docs/SPECS/.

ArchivoSkill NamePropósito y Momento de UsoSpec-0.mdproject-setupConfiguración de compilador, auditoría de dependencias y optimización de binarios.Spec-1.mdmap-gen-expertGeneración procedimental de mapas finitos y validación de conectividad.Spec-2.mdecs-optimizerDiseño de componentes agrupados (Data-Oriented) y auditoría de caché.Spec-3.mdbiological-engineSimulación de ADN, herencia mendeliana e infección por proximidad.Spec-4.mdkingdom-geopoliticsGestión de reinos, diplomacia masiva y Swarm AI (Boids).Spec-5.mdpersistence-architectSerialización binaria con Bincode y streaming dinámico de chunks desde disco.Spec-6.mdsensory-expertVisión (FOV) mediante Shadowcasting y lógica del ciclo día-tarde-noche.Spec-7.mdprogression-specialistProgresión de niveles y jerarquía de habilidades de clase (cada 20 niveles).Spec-8.mdui-orchestratorMáquina de estados (RunState), gestión de menús y capas de consola ASCII/Tiles.Spec-9.mdmaterial-engineerCiencia de materiales para objetos e interacciones químicas/biológicas (Blight).Spec-10.mdstate-integration-architectIntegración del WorldManager con el ciclo de vida de los estados y flujo de datos.Spec-11.mdregional-map-expertGeneración técnica de Parasangas (Drunkard's Walk) y conectividad regional.Spec-12.mdpathfinding-specialistNavegación eficiente mediante Dijkstra Maps para hordas y obstáculos.3. Reglas de Oro (Leyes del Proyecto)

Cualquier código generado debe obedecer estas restricciones para evitar el colapso del Celeron:

Zero-A Policy*: Prohibido usar A* para hordas o entidades de baja inteligencia. Usar Dijkstra Maps o Boids.


Data-Oriented Design (DOD): Priorizar el uso de Flat Vectors sobre estructuras anidadas o HashMap.


División Hot/Cold: Los datos de movimiento deben estar físicamente separados de los datos de historial/genealogía en el ECS.


Batching Paralelo: Las tareas de rayon deben procesarse en lotes (chunks) de 500 entidades para no saturar los 2 núcleos con el overhead de hilos.


Abstracción Temporal: Los sistemas pesados (diplomacia, infección) deben ejecutarse cada X ticks, nunca cada frame.

4. Protocolo de Carga para la IA


Nivel 1 (Metadata): Al iniciar, lee este GEMINI.md para conocer las restricciones.

Nivel 2 (Selección): Identifica qué Spec-X es necesario para la tarea actual.

Nivel 3 (Validación): Antes de entregar código, verifica si cumple los criterios de "Cero Desperdicio" (Zero-Waste).


