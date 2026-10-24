use super::Controller;
use gtk4::{gio, prelude::*};
use std::time::Duration;

pub(super) fn run(controller: &Controller) {
    // A dedicated connection gives the Shell extension an unambiguous lifetime/owner.
    let connection =
        gio::dbus_address_get_for_bus_sync(gio::BusType::Session, gio::Cancellable::NONE).and_then(
            |address| {
                gio::DBusConnection::for_address_sync(
                    &address,
                    gio::DBusConnectionFlags::AUTHENTICATION_CLIENT
                        | gio::DBusConnectionFlags::MESSAGE_BUS_CONNECTION,
                    None,
                    gio::Cancellable::NONE,
                )
            },
        );
    let Ok(connection) = connection else {
        controller.fail("Bus de session indisponible / Session bus unavailable");
        return;
    };
    connection.set_exit_on_close(false);
    let mut previously_enabled = false;
    loop {
        let request = controller.request.lock().unwrap().clone();
        if request.enabled || previously_enabled {
            let payload = serde_json::json!({
                "request": request,
                "appearance": controller.bridge.appearance(),
            })
            .to_string();
            let result = connection.call_sync(
                Some("org.gnome.Shell"),
                "/fr/jeromelab/KeycastBridge/Presentation",
                "fr.jeromelab.KeycastBridge.Presentation",
                "Update",
                Some(&(payload,).to_variant()),
                None,
                gio::DBusCallFlags::NO_AUTO_START,
                750,
                gio::Cancellable::NONE,
            );
            match result {
                Ok(reply) => {
                    if let Some((message,)) = reply.get::<(String,)>() {
                        if message == "stopped" && request.enabled {
                            controller.enable(false);
                            controller.bridge.stop();
                        }
                        *controller.status.lock().unwrap() = message;
                    }
                }
                Err(_) => controller.fail("Activer l’extension GNOME Keycast Bridge / Enable the Keycast Bridge GNOME extension"),
            }
        }
        previously_enabled = request.enabled;
        if controller.stopped() {
            // The connection closing also causes immediate cleanup in the extension.
            let _ = connection.close_sync(gio::Cancellable::NONE);
            break;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}
