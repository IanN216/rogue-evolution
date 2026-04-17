name: ui-orchestrator
description: Experto en gestión de estados de interfaz y consolas virtuales de bracket-lib para "Rogue-Evolución". Diseña menús, creación de personajes y el HUD ASCII. Actívalo con: "crear menú de inicio", "diseñar HUD de juego" o "gestionar estados de aplicación".
allowed-tools: [ls, cat, cargo]
Spec #8: UI Orchestration & State Management
1. Máquina de Estados (RunState)

Uso de un enum central para desacoplar el renderizado del proceso de simulación:

    MainMenu, CharacterCreation, MapGen, InGame, Laboratory (para experimentos con sujetos). Este patrón asegura que solo se procesen los recursos necesarios para el estado actual.

2. Capas de Consola (ASCII a Tiles)

    El sistema debe usar capas de bracket-lib (layering) para separar el mapa del HUD de estadísticas.

    Future-Proof: Todas las llamadas de dibujo deben referenciar un ID de tile para permitir el cambio de ASCII a pixel art de 32x32 sin tocar la lógica de los estados.