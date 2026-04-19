#!/bin/bash
# Agente Supervisor Rogue-Evolution v5 (Mapeo de Referencias)
# Uso: exec ./master-build.sh docs/SPECS/Specs-1.1.md

SPEC_FILE=$1
SUCCESS_KEYWORD="SISTEMA 100% OPERATIVO"

# Mapeo de Spec -> Referencia Técnica (Ahorro de Tokens)
case "$SPEC_FILE" in
    *"Specs-1.1"*|*"Specs-1"*) REF="boundary-ca-maps.md" ;;
    *"Specs-3"*)  REF="evolution-genetics.md" ;;
    *"Specs-12"*) REF="dijkstra-navigation.md" ;;
    *"Specs-8.1"*) REF="dual-layer-persistence.md" ;;
    *) REF="map-blocking-integrity.md" ;; 
    *"Specs-15"*) REF="dual-layer-persistence.md" ;;# Referencia por defecto
esac

limpiar_y_cerrar() {
    echo -e "\n🛑 Tarea terminada. Cerrando shell..."
    sleep 1
    kill -SIGHUP $PPID
    exit 0
}

trap limpiar_y_cerrar SIGINT

if [ -z "$SPEC_FILE" ]; then echo "❌ Error: Especifica un Spec."; exit 1; fi

echo "🚀 Construyendo: $SPEC_FILE"
echo "📚 Referencia activa: $REF"

for i in {1..5}
do
    echo "🔄 CICLO $i..."
    # 1. TRABAJO HEADLESS
    gemini run "$SPEC_FILE" --checkpoint --headless
    
    # 2. VALIDACIÓN TÉCNICA (Rápida en Celeron)
    if ! cargo check; then
        gemini "Error en 'cargo check'. Corrige la sintaxis en $SPEC_FILE." --checkpoint --headless
        continue
    fi

    # 3. AUDITORÍA CON CRUCE DE REFERENCIA
    echo "🔍 Auditando integridad con $REF..."
    # Aquí es donde ocurre la magia: forzamos a leer la referencia específica
    REPORTE=$(gemini "Usa los skills 'system-auditor' y 'rogue-scholar'. Lee la referencia 'docs/SKILLS/reference/$REF' y valida si el código de $SPEC_FILE cumple con la teoría al 100%. Responde SOLO con '$SUCCESS_KEYWORD' o los fallos técnicos." --checkpoint --headless)

    if [[ "$REPORTE" == *"$SUCCESS_KEYWORD"* ]]; then
        echo "✅ ÉXITO: Sistema validado contra investigación de NotebookLM."
        limpiar_y_cerrar
    else
        echo "⚠️ Fallo Teórico: $REPORTE"
        gemini "La implementación no cumple con la referencia $REF. Detalle: $REPORTE. Corrige el código y elimina esqueletos TODO." --checkpoint --headless
    fi
done

echo "🛑 Límite alcanzado."
limpiar_y_cerrar