#!/bin/bash
# Agente Supervisor Rogue-Evolution v4 (Privilegios Totales)
# Uso: exec ./master-build.sh docs/SPECS/Specs-8.1.md

SPEC_FILE=$1
SUCCESS_KEYWORD="SISTEMA 100% OPERATIVO"

# Función de cierre limpio
limpiar_y_cerrar() {
    echo -e "\n🛑 Interrupción detectada. Limpiando caché y cerrando shell..."
    sleep 1
    kill -SIGHUP $PPID
    exit 0
}

# Capturamos Ctrl+C (SIGINT) y fallos del sistema
trap limpiar_y_cerrar SIGINT SIGTERM

if [ -z "$SPEC_FILE" ]; then
    echo "❌ Error: Especifica un Spec."
    exit 1
fi

echo "🚀 Iniciando construcción con Privilegios Totales en: $SPEC_FILE"

for i in {1..5}
do
    echo "🔄 CICLO DE TRABAJO $i..."
    
    # 1. TRABAJO: --headless evita las preguntas de "Allow?"
    gemini run "$SPEC_FILE" --checkpoint --headless
    
    # 2. VALIDACIÓN TÉCNICA (Ahorra ciclos de CPU Celeron)
    if ! cargo check; then
        echo "❌ Error de compilación. Enviando reporte detallado..."
        gemini "Corrige los errores de Rust reportados por 'cargo check' en $SPEC_FILE." --checkpoint --headless
        continue
    fi

    # 3. AUDITORÍA DE INTEGRIDAD (Spec-13)
    echo "🔍 Auditando..."
    REPORTE=$(gemini "Usa el skill system-auditor para validar $SPEC_FILE. Responde SOLO con '$SUCCESS_KEYWORD' o los archivos con placeholders/TODO." --checkpoint --headless)

    if [[ "$REPORTE" == *"$SUCCESS_KEYWORD"* ]]; then
        echo "✅ ÉXITO: Sistema validado al 100%."
        limpiar_y_cerrar
    else
        echo "⚠️ Hallazgos: $REPORTE"
        # Instrucción agresiva para forzar la unión de las "blocked calls" en map.rs
        gemini "Auditoría FALLIDA. Detecté placeholders en $REPORTE. Une las funciones de bloqueo en map.rs con el ECS y elimina el comentario 'Logic would go here' en main_menu.rs." --checkpoint --headless
    fi
done

echo "🛑 Límite de intentos alcanzado."
limpiar_y_cerrar