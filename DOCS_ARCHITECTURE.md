Esta es la documentación oficial de la **Parte 1** para el proyecto **"Ecosistema Rogue-Evolución"**. Está diseñada específicamente para un desarrollo en Rust orientado a hardware de bajos recursos (Intel Celeron N2806, 2 núcleos, 4GB RAM), priorizando la eficiencia extrema sin sacrificar la profundidad de simulación tipo *Cave of Qud* o *Elona*.

---

# Documentación Técnica: Parte 1 — Cimientos y Arquitectura

## 1. Librerías y Herramientas (Stack Tecnológico)
Para garantizar el rendimiento en un procesador de doble núcleo y poca memoria, el proyecto se construye sobre librerías de bajo impacto y alto control manual:

* [cite_start]**`hecs`**: Un motor de **Sistemas de Entidades-Componentes (ECS)** de alto rendimiento y minimalista[cite: 3]. [cite_start]Se elige porque ofrece una clausura de dependencias muy pequeña y prioriza la velocidad de travesía, lo cual es vital para procesar 10,000 entidades sin saturar el CPU[cite: 4, 6, 8].
* [cite_start]**`bracket-lib`**: Provee las consolas virtuales necesarias para el estilo Roguelike tradicional[cite: 278, 279]. [cite_start]Es ideal para el hardware objetivo porque permite manejar tanto ASCII como tilesets de 32x32 de forma eficiente y es agnóstica a la arquitectura del juego[cite: 283, 286].
* [cite_start]**`rayon`**: Marco de computación paralela que utilizará automáticamente los 2 núcleos del Celeron para tareas masivas (como la IA de hordas) de forma segura[cite: 252, 265, 273].
* **`serde` & `bincode`**: Para la persistencia de datos. [cite_start]`bincode` es un formato binario ultra-compacto y rápido, esencial para guardar el estado del mundo masivo en un disco lento[cite: 8].

---

## 2. Estructura de Carpetas (Organización del Proyecto)
La organización sigue un patrón modular que separa la simulación pura del renderizado y la gestión de estados.

```text
src/
├── main.rs                 # Inicialización y bucle de estados (State Pattern).
├── states/                 # Gestión de Menús y flujo de la aplicación.
│   ├── main_menu.rs        # Lógica del menú de inicio.
│   ├── map_gen_screen.rs   # Visualización de la generación del mundo.
│   └── ingame.rs           # Bucle principal de simulación y entrada.
├── core/                   # El motor del mundo.
│   ├── map.rs              # Rejilla local, colisiones y tipos de terreno.
│   ├── world_map.rs        # Gestión del mapa global (Parasangas).
│   └── spawner.rs          # Factoría de entidades (monstruos, ciudadanos, reyes).
├── components/             # Definiciones de datos puros (Grouped Structs).
│   ├── stats.rs            # Atributos (Inteligencia, Magia, etc.).
│   ├── genetics.rs         # ADN, habilidades de raza/clase, infecciones.
│   └── identity.rs         # Títulos, lealtad a reinos y genealogía.
├── systems/                # Lógica procesada por Rayon.
│   ├── ai/                 # IAs estratificadas (Líderes vs Hordas).
│   ├── biology/            # Infección, evolución biológica y hambre.
│   └── kingdom/            # Tick diplomático y gestión de territorio.
└── utils/                  # Herramientas de soporte.
    ├── fov.rs              # Cálculos de visibilidad y luz.
    └── persistence.rs      # Guardado en disco mediante Bincode.
```

---

## 3. Algoritmos Críticos
Se han seleccionado algoritmos que optimizan el uso de CPU mediante la reducción de complejidad computacional.

* [cite_start]**Dijkstra Maps (Navegación)**: Utilizado para que las hordas e invasiones "huelan" objetivos sin que cada una de las 200 entidades calcule una ruta individual (A*), ahorrando ciclos masivos de CPU[cite: 171].
* **Spatial Hashing (Colisiones e Infección)**: En lugar de comparar 10,000 entidades entre sí ($O(n^2)$), se dividen en celdas de cuadrícula. La infección y las colisiones solo se verifican entre entidades en la misma celda ($O(n)$).
* **Symmetric Shadowcasting (FOV)**: Calcula la visión del jugador y monstruos inteligentes de forma rápida y visualmente limpia para el ciclo día/noche.
* **Ruido Simplex & Autómatas Celulares**: Para generar el mapa finito inicial de forma orgánica, evitando el uso de RAM excesiva que requieren otros algoritmos de generación infinita.

---

## 4. Datos del Sistema (Estructuras de Componentes)
[cite_start]Para evitar la fragmentación de memoria (que ralentiza los Celeron), los datos no se dispersan, sino que se agrupan en **Bolsas de Datos**[cite: 63, 70, 143].

* **`BaseStats`**: Agrupa en un solo bloque de memoria: `ataque`, `defensa`, `inteligencia`, `magia`, `suerte`, `velocidad`.
* **`AbilityRegistry`**: Un componente único con vectores internos para `RaceAbilities` (incluyendo genética), `ClassAbilities` (cada 20 niveles) y `SpecialAbilities`.
* **`Identity`**: Almacena el ID del reino, los títulos (como `Asesino de Dragones`) y los IDs de los padres para la genealogía.
* [cite_start]**División Hot/Cold**: Los datos de movimiento (posición) están separados de los datos de historial (genealogía) para que el CPU no cargue información innecesaria en el caché[cite: 124, 125, 143].



---

## 5. Puntos Críticos y Advertencias
* **Advertencia de Fragmentación**: Nunca crees un componente individual para una habilidad única. Agrúpalas en el `AbilityRegistry`. [cite_start]Demasiados arquetipos en `hecs` destruirán el rendimiento en el Celeron[cite: 232, 233].
* **Punto Crítico de IA**: Las entidades con `Inteligencia < 30` **no deben** usar Pathfinding complejo. Deben usar *Steering Behaviors* (Boids) para seguir a un líder, reduciendo la carga del CPU en un 90%.
* **Persistencia**: El streaming de chunks es obligatorio. El juego debe serializar a disco las entidades que no estén en el radio de visión del jugador para mantener el uso de RAM por debajo de 1GB.

---

## 6. Planificación del MVP (Mínimo Producto Viable)
El desarrollo se dividirá en fases para asegurar que el sistema sea estable y escalable desde el primer día:

1.  **Hito 1: Motor de Mundo**: Implementar el mapa finito básico con `bracket-lib`, el sistema de chunks y un jugador que pueda moverse en ASCII.
2.  **Hito 2: Ecosistema Base**: Introducir a los primeros 500 monstruos usando `hecs`. Implementar el hambre y la muerte básica.
3.  **Hito 3: El Sistema de Infección**: Implementar la raza de la "Plaga" y el algoritmo de intercambio de habilidades genéticas por proximidad física.
4.  **Hito 4: Inteligencia y Reinos**: Añadir el sistema de `Intelligence` y la lógica de reinos que colapsan. Implementar la primera invasión de horda (100 unidades).
5.  **Hito 5: Escalamiento y Pulido**: Optimizar con `rayon`, añadir el sistema de clases (cada 20 niveles) y preparar el soporte para sprites de 32x32.

---
Parte 2:
Esta es la **Parte 2** de la documentación técnica, centrada en el motor biológico, el sistema de herencia genética y la mecánica de infección por razas. Este sistema permite que el ecosistema evolucione de forma orgánica y emergente sin saturar el procesador Intel Celeron N2806.

---

# Documentación Técnica: Parte 2 — Motor Biológico y Genética de Infección

## 1. Librerías y Compatibilidad (Enfoque Biológico)
Para procesar la evolución de 10,000 entidades, aprovechamos la eficiencia de memoria de `hecs` y el procesamiento paralelo de `rayon`:

* **`hecs`**: Utilizada para gestionar arquetipos de criaturas. [cite_start]Permite realizar travesías rápidas sobre entidades que comparten componentes biológicos (como hambre o ADN) sin tocar datos irrelevantes[cite: 3, 6].
* [cite_start]**`rayon`**: Esencial para distribuir el cálculo de actualizaciones biológicas (metabolismo, mutación e infección) entre los 2 núcleos del Celeron, maximizando la utilización de threads sin bloqueos manuales[cite: 265, 273].
* [cite_start]**`rand`**: Para gestionar las probabilidades de mutación y eventos aleatorios en la genética[cite: 1].

---

## 2. Estructura de Carpetas (Módulos de Biología)
La lógica biológica reside en un submódulo especializado para facilitar la escalabilidad de rasgos:

```text
src/
├── components/
│   └── genetics.rs         # Definición de ADN, genes dominantes y habilidades.
├── systems/
│   └── biology/            # Cerebro biológico del ecosistema.
│       ├── metabolism.rs   # Procesamiento de hambre, sed y energía.
│       ├── infection.rs    # Lógica de la raza Plaga e intercambio de habilidades.
│       ├── evolution.rs    # Triggers de mutación y anomalías del entorno.
│       └── reproduction.rs # Herencia de rasgos y genealogía.
└── core/
    └── biological_constants.rs # Parámetros globales de equilibrio (tasas de hambre, etc.).
```

---

## 3. Algoritmos Biológicos y Justificación
Para mantener la fluidez en hardware limitado, se han seleccionado algoritmos de baja complejidad temporal:

### 3.1. Herencia de Rasgos (Mendelismo Simplificado)
* **Algoritmo**: Combinación aleatoria de vectores de rasgos con factor de mutación.
* **Ubicación**: `systems/biology/reproduction.rs`.
* [cite_start]**Por qué**: Permite que las crías hereden valores estadísticos (como velocidad o percepción) y habilidades de raza con variaciones pequeñas[cite: 264]. [cite_start]Esto genera selección natural: solo los rasgos exitosos sobreviven para reproducirse[cite: 264].

### 3.2. Sincronización de Pulso (Infección por Proximidad)
* **Algoritmo**: *Spatial Hashing* de proximidad física.
* **Ubicación**: `systems/biology/infection.rs`.
* **Por qué**: En lugar de que cada infectado busque a quién contagiar cada frame ($O(n^2)$), el sistema divide el mapa en rejillas. Solo las entidades en la misma celda revisan si pueden compartir habilidades de raza. [cite_start]Esto reduce la carga del CPU un 90%[cite: 141].

### 3.3. Árbol de Evolución Adaptativo
* **Algoritmo**: Chequeo de Umbrales de Exposición (Threshold Checking).
* **Ubicación**: `systems/biology/evolution.rs`.
* **Por qué**: Cada entidad rastrea su exposición a "anomalías" (como el cadáver de dragón). [cite_start]Al superar un valor $X$, el sistema dispara una transformación de arquetipo en `hecs`, lo cual es más eficiente que tener IFs constantes en la IA[cite: 63].

---

## 4. Datos del Sistema (Modelado de ADN y Habilidades)
[cite_start]El sistema utiliza una estructura de **Datos Agrupados** para maximizar el uso del caché del Celeron[cite: 124, 143].

### 4.1. El Componente `Genetics`
```rust
struct Genetics {
    dna: Vec2,               // Estadísticas heredables (Velocidad, Tamaño).
    race_abilities: Vec<u8>, // IDs de habilidades de raza compartidas.
    class_abilities: Vec<u8>,// Habilidades de clase (Humanos/Elfos/Enanos cada 20 niveles).
    exposure_level: f32,     // Acumulación de anomalías para evoluciones secretas.
}
```


### 4.2. Modelado de la Raza Plaga
La plaga no es solo un estado, es una **raza** con un comportamiento de colmena:
* **Compartición**: Al estar físicamente cerca, un infectado puede "donar" una habilidad genérica de su lista a otra entidad infectada.
* **Infección de Estructuras**: Un cadáver (ej. dragón) se convierte en una entidad estática con el componente `InfectionSource`, infectando a cualquiera que intente recolectar recursos de él.

---

## 5. Puntos Críticos y Advertencias
* **Advertencia de Consumo**: El cálculo de visibilidad y proximidad para 10,000 entidades puede causar picos de lag. Es crítico ejecutar el sistema de infección cada **X ticks** (ej. 30 o 60) en lugar de cada frame.
* **Punto Crítico de Memoria**: Mantén las listas de habilidades (`race_abilities`) pequeñas. Si cada monstruo tiene 50 habilidades únicas, los vectores crecerán demasiado y consumirán los 4GB de RAM rápidamente.
* **Advertencia de Rayon**: Dado que solo hay 2 núcleos, no satures el sistema con tareas paralelas triviales. [cite_start]Agrupa el procesamiento de 500 entidades por cada "job" de Rayon para reducir el overhead de gestión de hilos[cite: 273].

---

## 6. Planificación del MVP (Fase Biológica)
Para asegurar que el sistema sea escalable hacia la complejidad de *Elona/Elin*, se propone este desarrollo por etapas:

1.  [cite_start]**Hito 1: Ciclo Vital**: Implementar componentes de hambre y metabolismo que afecten a la salud[cite: 264].
2.  [cite_start]**Hito 2: Herencia Genética**: Crear el sistema de reproducción donde las crías obtengan rasgos de velocidad y color de sus padres con mutaciones[cite: 264].
3.  **Hito 3: Prototipo de Infección**: Crear el componente `InfectionSource` (el cadáver infectado) y la transferencia de una habilidad simple por proximidad.
4.  **Hito 4: Evolución por Entorno**: Implementar las "Zonas de Anomalía" que cambien permanentemente el arquetipo de un monstruo tras una exposición prolongada.
5.  **Hito 5: Nivelación y Clases**: Añadir el sistema de niveles y el trigger que otorga una habilidad de clase cada 20 niveles solo a las razas correspondientes.

---
Parte 3:
Esta es la **Parte 3** de la documentación técnica, centrada en la gestión de reinos, la diplomacia a gran escala y la inteligencia artificial de hordas optimizada para procesadores de doble núcleo.

---

# Documentación Técnica: Parte 3 — Reinos, Diplomacia e IA de Hordas

## 1. Librerías y Compatibilidad (Geopolítica y Masa)
Para gestionar la interacción de hasta 10,000 habitantes distribuidos en múltiples reinos, se utilizan herramientas que permiten el procesamiento por lotes y la ejecución en segundo plano:

* [cite_start]**`hecs`**: Permite organizar a los habitantes en arquetipos según su reino y rol (ej. Soldado, Aldeano), facilitando consultas masivas para el sistema de diplomacia[cite: 6, 7].
* [cite_start]**`rayon`**: Crucial para paralelizar el sistema de "enjambre" (Swarm AI) en las invasiones, permitiendo que el segundo núcleo del Celeron maneje el movimiento de 200 soldados mientras el primero gestiona al jugador[cite: 145, 162, 265].
* [cite_start]**`bracket-lib`**: Se encarga de mostrar el estado del mapa global y los eventos de invasión mediante consolas virtuales ligeras que no consumen recursos excesivos de GPU[cite: 283, 284].

---

## 2. Estructura de Carpetas (Módulos de Reinos y Hordas)
La lógica se organiza para separar la simulación de "alta fidelidad" (cerca del jugador) de la simulación "abstracta" (lejos del jugador):

```text
src/
├── components/
[cite_start]│   └── kingdom.rs          # Componentes de recursos, orden, corrupción y lealtad. [cite: 69, 105]
├── systems/
│   ├── ai/
│   │   ├── swarm.rs        # Algoritmos de movimiento de horda (Boids).
[cite_start]│   │   └── pathfinding.rs  # Dijkstra Maps y navegación eficiente. [cite: 284]
│   └── kingdom/
│       ├── diplomacy.rs    # Tick maestro de 30 días y tratados.
│       ├── collapse.rs     # Lógica de caída de reinos y diáspora.
│       └── economy.rs      # Gestión de recursos del reino.
└── core/
    └── faction_data.rs     # Definiciones estáticas de cada raza y cultura.
```

---

## 3. Algoritmos de Gestión Masiva y Justificación

### 3.1. Tick Diplomático Maestro (Abstracción Temporal)
* **Algoritmo**: Evaluación Diferida de Estado.
* **Ubicación**: `systems/kingdom/diplomacy.rs`.
* **Por qué**: Calcular tratados y guerras para 10 reinos cada frame saturaría el CPU. El sistema realiza una revisión profunda solo cada 30 días de juego, decidiendo si un reino debe invadir a otro basándose en sus recursos y niveles de corrupción.

### 3.2. Movimiento de Horda (Swarm Intelligence)
* [cite_start]**Algoritmo**: *Steering Behaviors* (Boids) con Líder de Horda. [cite: 257]
* **Ubicación**: `systems/ai/swarm.rs`.
* **Por qué**: En lugar de calcular rutas individuales (A*) para 200 soldados en una invasión, solo el "Líder" calcula la ruta completa. [cite_start]Los demás miembros ejecutan funciones matemáticas simples de separación y alineación respecto al líder, reduciendo la carga de CPU en un 90%[cite: 122, 123, 143].

### 3.3. Colapso y Sujetos de Experimento
* [cite_start]**Algoritmo**: Transferencia de Posesión de Entidades. [cite: 114, 115]
* **Ubicación**: `systems/kingdom/collapse.rs`.
* [cite_start]**Por qué**: Cuando un reino cae, las entidades no se borran; se les quita su `KingdomMember` y se les asigna uno nuevo según la cultura del conquistador (ej. `Slave`, `ExperimentSubject`, o `Refugee`), cambiando dinámicamente su comportamiento en el ECS[cite: 119, 121].

---

## 4. Datos del Sistema (Modelado de Reinos)
[cite_start]Para optimizar el uso de los 4GB de RAM, los reinos se modelan como entidades macro con datos condensados[cite: 63, 65].

### 4.1. El Componente `KingdomState`
Este componente almacena la salud política y económica del reino:
* [cite_start]**Recursos**: Energía/Comida total acumulada por los ciudadanos. [cite: 259]
* **Corrupción**: Nivel de anomalía que puede transformar a los ciudadanos en mutantes o criminales.
* **Orden**: Probabilidad de que ciudadanos con alta `Intelligence` inicien una rebelión o sigan recordando a su antiguo rey.

### 4.2. Logística de Invasión
* [cite_start]**Tandas de Invasión**: El sistema genera oleadas de 100-200 unidades cada vez. [cite: 141]
* [cite_start]**Sincronización de Datos**: Solo se activan los componentes de IA completa cuando el jugador entra en la misma "Parasanga" (zona local) que la horda; de lo contrario, la invasión se procesa como un cambio numérico de territorio en el disco. [cite: 191]

---

## 5. Puntos Críticos y Advertencias
* **Advertencia de "Hordas Huérfanas"**: Si un Líder de Horda muere, el sistema debe asignar inmediatamente a otro líder para evitar que los 200 soldados se queden estáticos o causen errores de referencia en el ECS.
* **Punto Crítico de Rendimiento**: No utilices `A*` para unidades de bajo nivel de inteligencia. [cite_start]El uso masivo de navegación compleja en un Celeron provocará "stuttering" (tirones) en el renderizado de `bracket-lib`[cite: 282, 285].
* [cite_start]**Persistencia de Diplomacia**: Asegúrate de que los tratados y el historial de guerras se guarden en archivos de texto pequeños (JSON o binario) independientes de las entidades, para que el historial del mundo sea accesible sin cargar los 10,000 ciudadanos a la RAM. [cite: 238]

---

## 6. Planificación del MVP (Fase de Reinos)
Esta fase transforma la simulación biológica de la Parte 2 en un juego geopolítico:

1.  **Hito 1: Estructura de Reinos**: Definir los componentes de lealtad y los recursos básicos de cada facción.
2.  [cite_start]**Hito 2: Simulación de Hordas**: Implementar el comportamiento de enjambre (Boids) con un grupo de 50 entidades siguiendo a un líder. [cite: 252, 273]
3.  **Hito 3: El Ciclo de 30 Días**: Programar el tick maestro que evalúa el estado de los reinos y dispara eventos de guerra o paz.
4.  **Hito 4: Sistema de Conquista**: Implementar el cambio de cultura y la creación de "Sujetos de Experimento" tras el colapso de un reino.
5.  [cite_start]**Hito 5: Invasión en Tiempo Real**: Sincronizar el movimiento de una horda invasora a través del mapa global hasta que llegue a la zona local del jugador. [cite: 171, 190]
