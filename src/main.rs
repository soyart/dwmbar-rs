mod brightness;
mod clock;
mod sysfs;
use std::time::{
    Duration,
    Instant,
};

// Bar is our text-based status bar.
// It heavily relies on String as means of abstraction
struct Bar<'a> {
    // title of the whole bar
    title: &'a str,
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
fn run(title: &str, mut timers: Vec<Timer>) {
    let mut bar = Bar {
        title,
        values: timers.iter().map(|t| (t.key, initializing())).collect(),
    };

    let mut lasts: Vec<String> =
        timers.iter().map(|_| String::default()).collect();

    loop {
        let now = Instant::now();
        for (i, timer) in timers.iter_mut().enumerate() {
            let mut value = lasts.get(i).unwrap().to_owned();
            // Run all due timers
            if now >= timer.next_fire {
                let result = (timer.action)();
                if result.as_str() == value.as_str() {
                    continue;
                }
                value = result;
                lasts.insert(i, value.clone());
                bar.update(timer.key, value);
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
            key: "brightness",
            next_fire: now,
            interval: Duration::from_secs(3),
            action: brightness::get,
        },
        Timer {
            key: "clock",
            next_fire: now,
            interval: Duration::from_millis(500),
            action: clock::get,
        },
    ];

    run("dwmbar-rs", timers);
}

fn initializing() -> String {
    return String::from("initializing...");
}
