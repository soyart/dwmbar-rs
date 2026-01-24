mod brightness;
use std::time::{Duration, Instant};

// Bar is our text-based status bar.
// It heavily relies on String as means of abstraction
struct Bar<'a> {
    // title of the whole bar
    title: String,
    // vector of (key, value)
    values: Vec<(&'a str, String)>,
}

impl<'a> std::fmt::Display for Bar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)?;
        for (key, value) in &self.values {
            write!(f, " | {}: {}", key, value)?;
        }
        Ok(())
    }
}

impl<'a> Bar<'a> {
    fn update(&mut self, key: &'a str, value: String) {
        let current = self.to_string();
        for t in &mut self.values {
            if t.0 != key {
                continue;
            }
            t.1 = value.clone();
        }
        let updated = self.to_string();
        if current != updated {
            println!("{updated}");
        }
    }
}

struct Timer<'a> {
    key: &'a str,
    next_fire: Instant,
    interval: Duration,
    action: fn() -> String,
}

// run is a event-loop polling mechanism for our status bar.
fn run(mut timers: Vec<Timer>) {
    let mut bar = Bar {
        title: String::from("dwmbar-rs"),
        values: Vec::with_capacity(timers.len()),
    };
    for t in &timers {
        bar.values.push((t.key, initializing()));
    }

    loop {
        let now = Instant::now();
        for timer in &mut timers {
            // Run all due timers
            if now >= timer.next_fire {
                let result = (timer.action)();
                bar.update(timer.key, result);
                timer.next_fire = now + timer.interval;
            }
        }

        // Find the soonest next timer
        let next = timers.iter().map(|t| t.next_fire).min().unwrap();
        // Sleep until then (no busy-waiting!)
        std::thread::sleep(next - Instant::now());
    }
}

fn main() {
    let now = Instant::now();
    let timers = vec![
        Timer {
            key: "key1",
            next_fire: now,
            interval: Duration::from_secs(1),
            action: || String::from("key1 value"),
        },
        Timer {
            key: "key2",
            next_fire: now,
            interval: Duration::from_secs(2),
            action: || String::from("key2 value"),
        },
        Timer {
            key: "brightness",
            next_fire: now,
            interval: Duration::from_secs(3),
            action: || {
                let brightness = brightness::Brightness(vec![(100, 80), (100, 60), (255, 67)]);
                brightness.to_string()
            },
        },
    ];

    run(timers);
}

fn initializing() -> String {
    return String::from("initializing...");
}
