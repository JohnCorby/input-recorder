//! how we show shit to the user

use notify_rust::Notification;

/// blocks
pub fn msgbox(str: &str) {
    msgbox::create("input-recorder", str, msgbox::IconType::None).unwrap();
}

/// doesn't block
pub fn notify(str: &str) {
    Notification::new()
        .summary("input-recorder")
        .body(str)
        .show()
        .unwrap();
}
