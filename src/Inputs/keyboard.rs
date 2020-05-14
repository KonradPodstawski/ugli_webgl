pub fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    console!(
        log,
        "Key: ",
        key,
        ", location: ",
        location,
        ", pressed: ",
        is_pressed
    );
    true
}

pub fn actual_key_debug() {
    document().add_event_listener(move |event: KeyDownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        };
    });
}
