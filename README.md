# BetterWhatsApp (Tauri) - Linux

Cliente de WhatsApp para Linux basado en Tauri.

Características:

- Ventana que carga `https://web.whatsapp.com`
- Icono en la bandeja del sistema (tray) con menú (Abrir/Ocultar, Recargar, Toggle DevTools, Siempre encima, Salir)
- **Notificaciones nativas**: La app monitorea cambios en el contador de mensajes sin leer del título
- **Badge de mensajes no leídos**: Actualiza el título de la ventana con el contador (ej: "BetterWhatsApp (3)")
- **Minimizar a tray al cerrar**: Al hacer click en cerrar, la app se minimiza a la bandeja en lugar de cerrarse
- Opciones y recomendaciones para modo bajo consumo (desactivar aceleración por GPU, minimizar a bandeja, no iniciar en segundo plano)

Esto es un esqueleto inicial. Para desarrollar y empaquetar necesitarás tener instaladas las dependencias de Tauri y Rust.

Run (desarrollo):

```fish
# Instala dependencias frontend si las necesitas (ej. yarn/npm)
# cd /home/pico190/betterwhatsapp2
npm install
npm run tauri dev
```

Build (release):

```fish
npm run tauri build
```

Notas:

- Este proyecto carga WhatsApp Web; la autenticación la gestiona WhatsApp Web.
- Las notificaciones se muestran cuando cambias de aplicación o cuando el contador en el título cambia.
- **Minimizar a tray**: Cuando cierras la ventana con el botón X, se minimiza a la bandeja del sistema. Usa "Salir" en el menú del tray para cerrar completamente la app.
- Para ahorrar recursos, usa las opciones del menú o considera ejecutar con `--disable-gpu` en desarrollo.

Si quieres, implemento:

- Persistencia de sesión (profile/Storage persistente en Tauri)
- Integración de atajos globales (muestra/oculta con Ctrl+Alt+W)
- Autostart con opción para habilitar/deshabilitar
- Soporte de temas oscuro/claro
- Integración con native OS notifications (libnotify en Linux)

---

Creado automáticamente como esqueleto. Pide los siguientes pasos que quieras que implemente.
