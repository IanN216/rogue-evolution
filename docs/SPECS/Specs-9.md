name: material-engineer
description: Especialista en sistemas de objetos, materiales y química para "Rogue-Evolución". Implementa interacciones basadas en materiales (hierro, cristal, carne). Actívalo con: "diseñar sistema de materiales", "crear interacción química de objetos" o "configurar inventario persistente".
allowed-tools: [ls, cat, cargo]
Spec #9: Material Science & Itemization
1. Composición de Objetos

Los objetos no son clases rígidas, sino "bolsas de datos" con un componente MaterialType.

    Lógica: Un arma de "Cristal" tiene alta fragilidad pero mayor daño mágico en comparación con una de "Hierro".

2. Interacciones Biológicas

    Cadáveres: Funcionan como fuentes de recursos (Material: Organic).

    Infección: Si un objeto tiene contacto con una InfectionSource (ej. cadáver de dragón), adquiere un tag de Blighted, pudiendo infectar al portador.

3. Criterios de Calidad

    [ ] Minimal Allocation: ¿Se usan enums para los materiales para evitar el uso de Strings pesados?

    [ ] Stat Inheritance: ¿El daño del arma se calcula dinámicamente sumando la base del objeto + el multiplicador del material?