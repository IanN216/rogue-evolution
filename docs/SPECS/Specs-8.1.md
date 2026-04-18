# name: ui-persistence-master
# description: Menú principal con carga, borrado y validación de archivos.

## Requerimientos de UI
1. **Listado Dinámico**: Leer la carpeta `/saves` y listar los archivos `.bin` disponibles en la pantalla de `MainMenu`.
2. **Acciones**: Permitir seleccionar con flechas y borrar con la tecla `[D]`. Pedir confirmación antes de borrar.
3. **Seguridad de Carga**: Antes de usar `bincode` para deserializar, verificar un "Magic Number" o un ID de versión en la cabecera del archivo para evitar cargar archivos corruptos o de versiones viejas.

## Criterios de Aceptación (100%)
- [ ] El juego no crashea si el archivo de guardado está corrupto; muestra un error visual.
- [ ] La función de borrado elimina físicamente el archivo del disco.
- [ ] La carga de mapa lleva directamente al estado `RunState::InGame`.
