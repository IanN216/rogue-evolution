name: progression-specialist
description: Especialista en sistemas de niveles y jerarquía de clases para "Rogue-Evolución". Gestiona la progresión de humanos, elfos y enanos cada 20 niveles. Actívalo con: "implementar sistema de clases", "crear árbol de habilidades por nivel" o "definir progresión de jugador".
allowed-tools: [ls, cat, cargo]
Spec #7: Player Progression & Class Hierarchy
1. Sistema de Niveles

    Escalado: Solo las entidades con el tag Humanoid acceden al sistema de clases complejo.

    Habilidades de Clase: Cada 20 niveles, el sistema inserta un componente de habilidad único en el AbilityRegistry de la entidad.

2. Inserción Dinámica de Componentes

Para evitar la fragmentación, las nuevas habilidades se añaden al Vec<Ability> dentro del componente AbilityRegistry existente en lugar de crear componentes nuevos en hecs.
3. Criterios de Calidad

    [ ] Type Safety: ¿Se restringe el acceso a clases a las razas no permitidas (monstruos comunes)?

    [ ] Stat Scaling: ¿Las habilidades de clase aplican modificadores correctamente al bloque BaseStats?