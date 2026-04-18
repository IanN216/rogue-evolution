#!/bin/bash
# Agente Supervisor para Rogue-Evolution con Gestión de Señales
# Ejecución recomendada: exec ./master-build.sh docs/SPECS/Specs-8.1.md

SPEC_FILE=$1
MAX_INTENTOS=5
SUCCESS_KEYWORD="SISTEMA 100% OPERATIVO"

# Función de Cierre Forzoso (se activa con éxito, fallo o Ctrl+C)
cerrar_terminal() {
    echo -e "\n🛑 Finalizando procesos y cerrando terminal..."
    sleep 1
    kill -SIGHUP $PPID
    exit 0
}

# Captura de Ctrl + C (SIGINT)
trap cerrar_terminal SIGINT

if [ -z "$SPEC_FILE" ]; then
    echo "❌ Error: Especifica un Spec."
    exit 1
fi

echo "🚀 Iniciando construcción autónoma de: $SPEC_FILE"
echo "💡 (Presiona Ctrl+C en cualquier momento para salir y cerrar la shell)"

for ((i=1; i<=MAX_INTENTOS; i++))
do
    echo "--------------------------------------------------------"
    echo "🔄 INTENTO $i de $MAX_INTENTOS"
    
    # 1. Construcción: Uso de --headless para evitar preguntas de confirmación
    # Uso de --checkpoint para minimizar el envío de tokens repetidos
    gemini run "$SPEC_FILE" --checkpoint --headless
    
    # 2. Validación Técnica Rápida (Ahorro de tokens al no preguntar a la IA si hay errores de tipado)
    echo "🛠️ Verificando compilación (cargo check)..."
    if ! cargo check; then
        echo "❌ Error de sintaxis. Solicitando corrección inmediata..."
        gemini "Corrige los errores de 'cargo check' en el código de $SPEC_FILE." --checkpoint --headless
        continue
    fi

    # 3. Auditoría de Lógica e Integridad (Spec-13)
    echo "🔍 Auditando completitud del sistema..."
    REPORTE=$(gemini "Usa el skill system-auditor para validar $SPEC_FILE. Responde solo con '$SUCCESS_KEYWORD' o los errores." --checkpoint --headless)

    if [[ "$REPORTE" == *"$SUCCESS_KEYWORD"* ]]; then
        echo "✅ ÉXITO: Tarea completada al 100%."
        cerrar_terminal
    else
        echo "⚠️ Hallazgos del Auditor: $REPORTE"
        # Feedback agresivo para eliminar esqueletos vacíos
        gemini "Auditoría FALLIDA: $REPORTE. Basado en esto, completa todas las funciones vacías y uniones en mod.rs para $SPEC_FILE. No dejes comentarios TODO." --checkpoint --headless
    fi
done

echo "🛑 Límite de intentos agotado."
cerrar_terminal