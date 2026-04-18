#!/bin/bash
# Agente Supervisor para Rogue-Evolution
# Uso: ./master-build.sh docs/SPECS/Specs-1.1.md

SPEC_FILE=$1
MAX_INTENTOS=5
SUCCESS_KEYWORD="SISTEMA 100% OPERATIVO"

if [ -z "$SPEC_FILE" ]; then
    echo "Error: Debes especificar un archivo Spec (ej: docs/SPECS/Specs-1.1.md)"
    exit 1
fi

echo "🚀 Iniciando construcción de: $SPEC_FILE"

for ((i=1; i<=MAX_INTENTOS; i++))
do
    echo "--------------------------------------------------------"
    echo "🔄 INTENTO $i de $MAX_INTENTOS"
    
    # 1. Ejecutar la tarea usando el checkpoint para ahorrar tokens y RAM
    gemini run "$SPEC_FILE" --checkpoint
    
    echo "🔍 Iniciando Auditoría de Integridad..."
    
    # 2. Llamar al Skill Auditor (Spec-13) para verificar el trabajo
    # Guardamos el reporte en una variable
    REPORTE=$(gemini "Usa el skill system-auditor para evaluar si el código generado para $SPEC_FILE está terminado. Si no está al 100%, lista qué falta. Responde solo con '$SUCCESS_KEYWORD' o los errores.")

    # 3. Validar resultado
    if [[ "$REPORTE" == *"$SUCCESS_KEYWORD"* ]]; then
        echo "✅ ÉXITO: El sistema ha sido completado al 100%."
        exit 0
    else
        echo "❌ DEFICIENCIAS ENCONTRADAS:"
        echo "$REPORTE"
        echo "⚠️ Re-intentando corrección automática basado en la auditoría..."
        
        # 4. Feedback Loop: Mandamos el error de vuelta a Gemini para que lo arregle
        gemini "Auditoría fallida: $REPORTE. Basado en esto, completa todas las uniones faltantes y elimina comentarios TODO en el código de $SPEC_FILE."
    fi
done

echo "🛑 Se alcanzó el límite de intentos. El sistema requiere revisión manual."
