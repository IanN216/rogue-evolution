name: persistence-architect
description: Especialista en guardado binario y streaming de chunks para "Rogue-Evolución". Implementa la serialización con Bincode y la carga/descarga de Parasangas para liberar RAM. Actívalo con: "configurar persistencia de mundo", "implementar streaming de chunks" o "auditar guardado binario".
allowed-tools: [ls, cat, cargo, mkdir, rm]
Spec #5: World Persistence & Dynamic Streaming
1. Estrategia de Almacenamiento (Zero-Lag Save)

Debido a la limitación de 4GB de RAM, el sistema debe usar un modelo de Streaming de Chunks basado en la posición del jugador. No se permite mantener las 10,000 entidades activas simultáneamente en memoria.

    Formato: Uso obligatorio de bincode para serialización por su alta velocidad y bajo tamaño.

    Culling: Las entidades fuera del radio de 2 Parasangas del jugador se serializan y eliminan del hecs::World.

    Regiones: El mundo se divide en archivos de región individuales para evitar sobreescrituras masivas de un solo archivo de 500MB.

2. Ciclo de Carga/Descarga

    Trigger: Se activa cuando el jugador cruza el borde de una "Pantalla" local.

    Lazy Loading: Los datos de reinos lejanos (facciones, diplomacia) se cargan como "Fríos" (solo datos numéricos) y solo se activan como entidades físicas si hay un evento de invasión.

3. Criterios de Calidad

    [ ] Data Integrity: ¿Se validan los checksums al cargar una región desde el disco?

    [ ] Memory Ceiling: ¿El uso de RAM se mantiene bajo 1GB incluso con streaming activo?

    [ ] Thread-Safe I/O: ¿El guardado ocurre en un hilo separado de rayon para no congelar el renderizado?