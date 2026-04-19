#!/bin/bash
# Agente Supervisor Rogue-Evolution v6 (Refactorizado para Compatibilidad)
# Misión: Eliminar flags obsoletos y asegurar la integración de UI Dinámica.

SPEC_FILE=$1
SUCCESS_KEYWORD="SISTEMA 100% OPERATIVO"

# Mapeo de Spec -> Referencia Técnica (Ahorro de Tokens)
case "$SPEC_FILE" in
    *"Specs-1.1"*|*"Specs-1"*) REF="boundary-ca-maps.md" ;;
    *"Specs-3"*)  REF="evolution-genetics.md" ;;
    *"Specs-12"*) REF="dijkstra-navigation.md" ;;
    *"Specs-8.1"*) REF="dual-layer-persistence.md" ;;
    *"Specs-15"*) REF="dual-layer-persistence.md" ;;
    *"Specs-16"*) REF="dual-layer-persistence.md" ;;
    *) REF="map-blocking-integrity.md" ;;
esac

limpiar_y_cerrar() {
    echo -e "\n🛑 Proceso finalizado. Liberando recursos del Celeron..."
    sleep 1
    kill -SIGHUP $PPID
    exit 0
}

trap limpiar_y_cerrar SIGINT

if [ -z "$SPEC_FILE" ]; then 
    echo "❌ Error: Debes especificar un archivo de Spec (ej: docs/SPECS/Specs-16.md)."
    exit 1
fi

echo "🚀 Iniciando construcción de: $SPEC_FILE"
echo "📚 Cruzando con referencia teórica: $REF"

for i in {1..5}
do
    echo "🔄 CICLO DE TRABAJO $i/5..."
    
    # 1. EJECUCIÓN DEL SPEC
    # Se eliminan --checkpoint y --headless por incompatibilidad de entorno.
    # Se refuerza la regla de las 3 consolas y el centrado dinámico.
    gemini "Aplica los requerimientos de $SPEC_FILE. REGLA CRÍTICA: Implementa la limpieza de las 3 consolas (0..3) en cada tick y usa centrado dinámico absoluto (sw/2 - bw/2) para todos los recuadros de la UI."
    
    # 2. VALIDACIÓN SINTÁCTICA (Rápida en Celeron)
    if ! cargo check; then
        echo "⚠️ Fallo en cargo check. Solicitando reparación automática..."
        gemini "El código no compila. Corrige los errores de Rust en $SPEC_FILE basándote en la salida de 'cargo check' y asegúrate de no usar coordenadas hardcodeadas como 25 o 27."
        continue
    fi

    # 3. AUDITORÍA DE INTEGRIDAD (Spec-13 + Scholar)
    echo "🔍 Verificando integridad científica y visual..."
    # Se utiliza el prompt directo sin flags problemáticos
    REPORTE=$(gemini "Actúa como 'system-auditor' y 'rogue-scholar'. Valida el código contra 'docs/SKILLS/reference/$REF'. ¿Están las 3 consolas limpias? ¿Está la UI centrada dinámicamente en 170x48? Responde SOLO con '$SUCCESS_KEYWORD' o detalla los fallos.")

    if [[ "$REPORTE" == *"$SUCCESS_KEYWORD"* ]]; then
        echo "✅ ÉXITO TOTAL: Sistema validado contra la teoría y el hardware."
        limpiar_y_cerrar
    else
        echo "⚠️ Hallazgos encontrados: $REPORTE"
        gemini "Auditoría fallida. Corrige estos puntos: $REPORTE. Asegúrate de eliminar esqueletos TODO y restos visuales del menú anterior limpiando la Consola 2."
    fi
done

echo "🛑 Se alcanzó el límite de intentos sin validación completa."
limpiar_y_cerrar