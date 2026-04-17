name: state-integration-architect
description: Especialista en la integración del WorldManager con el ciclo de vida de bracket-lib. Gestiona el flujo de datos entre el ECS y los estados de RunState. Actívalo con: "integrar WorldManager en State", "conectar sistemas de juego al tick" o "configurar paso de referencias del mundo".
allowed-tools: [ls, cat, cargo]
Spec #10: World Integration & Data Flow
1. Propósito y Alcance

Este Skill actúa como el "pegamento" del sistema. Su misión es mover la instancia de WorldManager al interior del struct State en main.rs para que la simulación sea persistente y accesible para todos los sistemas de renderizado e IA.
2. Cambios Arquitectónicos Obligatorios

    State Ownership: El struct State debe poseer el WorldManager.

    Reference Passing: La función tick de cada módulo en src/states/ (como ingame::tick) debe aceptar una referencia mutable &mut WorldManager.

    Initialization: El mundo inicial y el jugador deben crearse en el estado CharacterCreation o MapGen e insertarse en el WorldManager antes de pasar a InGame.

3. Criterios de Calidad

    [ ] Borrows: ¿Se evita el uso de unsafe o RefCell para pasar el mundo entre estados?

    [ ] Frame Independence: ¿Se pasa el ctx.frame_time_ms a los sistemas de IA para mantener la velocidad constante?

    [ ] Memory Safety: ¿El WorldManager limpia adecuadamente las entidades antes de cambiar de estado (ej. volver al menú)?