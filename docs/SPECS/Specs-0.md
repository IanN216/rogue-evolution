---
name: project-setup
description: Inicializa y audita el entorno de desarrollo para "Rogue-Evolución". Úsalo para configurar el Cargo.toml optimizado para Celeron, crear la estructura modular de carpetas o ejecutar auditorías de configuración inicial. Actívalo con: "configurar compilador Rust para Celeron", "iniciar arquitectura base" o "auditar Cargo.toml".
allowed-tools: [ls, cat, cargo, mkdir, find, rm]
---

# Spec #0: Project Setup & Critical Audit

## 1. Propósito y Alcance
Este Skill es el arquitecto del sistema. Su misión es garantizar que el proyecto nazca con una configuración de "Zero-Waste" (Cero Desperdicio) y que el compilador de Rust genere el binario más eficiente posible para los 2 núcleos del Celeron N2806.

## 2. Configuración Maestra (Cargo.toml)
La IA debe asegurar que el perfil de `release` esté configurado para rendimiento extremo:

```toml
[package]
name = "rogue-evolution"
version = "0.1.0"
edition = "2021"

[dependencies]
hecs = "0.10"
bracket-lib = "0.8"
rayon = "1.8"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
rand = "0.8"

[profile.release]
opt-level = 3
lto = true          # Link Time Optimization masivo
codegen-units = 1   # Máxima optimización a costa de tiempo de compilación
panic = "abort"     # Reduce el tamaño del binario y mejora la velocidad
strip = true        # Elimina símbolos innecesarios para ahorrar espacio en disco