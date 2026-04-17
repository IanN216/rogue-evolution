name: sensory-expert
description: Especialista en sistemas de visión (FOV) e iluminación dinámica para "Rogue-Evolución". Implementa Symmetric Shadowcasting y el ciclo día/noche. Actívalo con: "implementar FOV", "crear ciclo día-noche" o "optimizar visibilidad de entidades".
allowed-tools: [ls, cat, cargo]
Spec #6: Sensory Engine & Chronometry
1. Sistema de Visión (FOV)

    Algoritmo: Symmetric Shadowcasting. Debe ser capaz de procesar la visión del jugador y entidades con Intelligence > 70 en menos de 1ms.

    Optimización: El FOV solo se recalcula si el jugador se mueve o el entorno cambia (destrucción de muros).

2. Ciclo Día-Tarde-Noche

    Mecánica: El radio de visión es un componente mutable que depende del reloj del juego.

        Día: Radio 20-30 celdas.

        Noche: Radio 3-5 celdas (gradiente de oscuridad aplicado mediante bracket-lib).

    Fuentes de Luz: Los objetos con el tag LightSource expanden el FOV localmente ignorando el ciclo global.

3. Test Obligatorio

    Simular el paso de 24 horas de juego.

    Validar que el radio de visión del jugador disminuya al llegar la noche y aumente al amanecer.