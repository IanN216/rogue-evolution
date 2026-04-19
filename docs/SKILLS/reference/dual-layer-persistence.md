# Referencia Técnica: Persistencia Híbrida (Performance + Debug)

Este documento define el protocolo de guardado y carga para "Rogue-Evolution", balanceando la velocidad del disco mecánico con la transparencia del desarrollo.

## 1. Capa de Producción: Bincode (Binary)
Para el flujo normal de juego en el hardware Celeron, el formato debe ser binario puro.
- **Razón**: `bincode` es significativamente más rápido que JSON en CPUs limitados porque no requiere parseo de strings complejos.
- **Checksum**: Se debe mantener el **XOR Checksum** actual para validar la integridad en microsegundos.

## 2. Capa de Debug: JSON Moddeable (Text)
El sistema debe permitir la exportación voluntaria del estado del mundo a archivos `.json` legibles.
- **Utilidad**: Permite al desarrollador editar manualmente la salud de un monstruo o la genética de una planta para probar casos de borde sin reprogramar el motor.
- **Optimización de Espacio**: Se DEBEN usar los atributos de `serde` para omitir valores por defecto y campos `None`, evitando archivos de texto de varios megabytes.

## 3. Implementación de "Magic Numbers"
Cada archivo guardado debe comenzar con una cabecera de 4 bytes (Magic Number) y 2 bytes de versión.
- **Seguridad**: Evita que el programa intente cargar un archivo de una versión vieja que causaría un `Panic` por cambios en las estructuras de datos.

## 4. Gestión de Archivos en el Celeron
Para evitar fragmentación en el disco mecánico:
- **No guardar todo en un archivo**: Dividir el mundo en archivos de "Región" (como ya está en `persistence.rs`).
- **Escritura Diferida**: Solo guardar la región que ha sufrido cambios lógicos importantes.

## 5. Criterios de Validación para el Auditor (Spec-13)
El Auditor debe marcar como **FALLO** si:
- El sistema de carga no verifica el **MAGIC_NUMBER** antes de procesar el archivo.
- No existe una herramienta de "Exportar a JSON" para depuración.
- Se detectan funciones de guardado que bloquean el hilo principal por más de 100ms (debe ser asíncrono o muy optimizado).