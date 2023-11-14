use std::env;
extern crate notify_rust;
use notify_rust::Notification;
extern crate chrono;
extern crate timer;
use std::sync::mpsc::channel;

fn main() {
    let args: Vec<String> = env::args().collect();
    let t = timer::Timer::new();
    let command = &args[1];

    match command.as_str() {
        "start" => start(t),
        "stop" => stop(),
        "resume" => resume(),
        "pause" => pause(),
        "restart" => restart(),
        _ => println!("Command not found"),
    }
}

fn start(t: timer::Timer) {
    let (tx, rx) = channel();
    Notification::new()
        .summary("Pomodoro")
        .body("Pomodoro started")
        .icon("dialog-information")
        .show()
        .unwrap();
    let _gaurd = t.schedule_with_delay(chrono::Duration::seconds(5), move || {
        let _ignore = tx.send(());
    });
    rx.recv().unwrap();
    Notification::new()
        .summary("Pomodoro")
        .body("Pomodoro ended")
        .icon("dialog-information")
        .show()
        .unwrap();
}

fn stop() {
    todo!();
}

fn resume() {
    todo!();
}

fn pause() {
    todo!();
}

fn restart() {
    todo!();
}
