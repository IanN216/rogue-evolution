Spec #12: Dijkstra Navigation & Obstacle Awareness
1. Propósito

Sustituir el cálculo de distancia euclidiana simple en src/systems/ai/swarm.rs por una navegación real que respete muros y optimice el movimiento de grupos masivos (Hordas).
2. Implementación Técnica

    Dijkstra Maps: Crear una rejilla de distancias desde un objetivo (ej. el Jugador o una Ciudad).

    Caching: El mapa de Dijkstra solo se recalcula si el objetivo se mueve fuera de su Parasanga actual o si el terreno cambia.

    Horde Movement: El "Líder de Horda" sigue el gradiente descendente del mapa. Los seguidores consultan el mapa de Dijkstra de su líder para ahorrar ciclos de CPU.

3. Criterios de Calidad

    [ ] Wall Collision: ¿Las entidades evitan caminar a través de tiles de Wall?

    [ ] Performance Check: ¿Calcular el mapa para una zona de 80x50 toma menos de 0.5ms?

    [ ] Batching: ¿Rayon procesa el movimiento de la horda consultando el mapa de Dijkstra en paralelo?