use enigo::{Enigo, MouseButton, MouseControllable};
use inputbot::{KeybdKey::*, *};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    println!("Grizzy Lemmings Sling Auto Clicker\n Press F6 enable/disable clicker");
    let clicker_is_active = Arc::new(AtomicBool::new(false));
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);

    thread::spawn(move || {
        let mut enigo = Enigo::new();
        let drag_duration = time::Duration::from_millis(40);

        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_down(MouseButton::Left);
                thread::sleep(drag_duration);

                enigo.mouse_move_relative(10, 120);
                thread::sleep(drag_duration);

                enigo.mouse_up(MouseButton::Left);
                thread::sleep(drag_duration);

                enigo.mouse_move_relative(-10, -120);
                thread::sleep(drag_duration);
            }
        }
    });

    F6Key.bind(move || {
        clicker_is_active.store(
            !clicker_is_active.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
    });

    handle_input_events();
}
