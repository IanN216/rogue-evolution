# Referencia Técnica: Algoritmo Genético y Evolución Mendeliana

Este documento define la implementación correcta del motor biológico para "Rogue-Evolution", basado en la investigación de simulaciones ABM (Agent-Based Models) en Rust.

## 1. Estructura del Genoma (DOD)
Para maximizar el rendimiento en el Celeron, el genoma no debe ser un objeto complejo, sino un `Flat Vector` de f32 (pesos) o u8 (rasgos binarios).

- **Cromosomas**: Un array fijo de genes que representan:
  - Velocidad base.
  - Radio de detección (Sensory).
  - Tasa metabólica (Gasto de energía).
  - Resistencia a la infección.

## 2. Herencia Mendeliana (Crossover)
La implementación debe alejarse de un simple "promedio" de valores. Debe usar un sistema de **Alelos Dominantes y Recesivos**:

1. **Selección de Progenitores**: Usar el método de "Torneo" (seleccionar 2 de los más aptos).
2. **Crossover por Punto**: Elegir un índice aleatorio en el vector genético. La descendencia toma los genes del Padre A hasta el punto `k` y los del Padre B después de `k`.
3. **Dominancia**: Cada gen debe tener un valor de "fuerza". Si el gen de resistencia del Padre A es dominante sobre el de el Padre B, el hijo hereda el de A.

## 3. Mutación Gaussiana
Para evitar el estancamiento evolutivo (mencionado en *Learning to Fly*), se debe aplicar una mutación:
- **Probabilidad**: 0.01 a 0.05.
- **Efecto**: Sumar un valor aleatorio basado en una distribución normal al peso del gen.

## 4. Presión de Selección (Fitness)
El valor de aptitud (`fitness`) debe ser acumulativo en cada tick:
- **Positivo**: Comer, infectar a otros, sobrevivir ciclos de tiempo.
- **Negativo**: Recibir daño, gastar energía sin comer.

## 5. Criterios de Validación para el Auditor
El Auditor (Spec-13) debe marcar como **FALLO** si:
- No existe una función `crossover` que combine dos genomas.
- La mutación es puramente aleatoria sin seguir una escala (debe ser incremental).
- Las entidades nacen con valores al azar en lugar de heredar los de sus padres.