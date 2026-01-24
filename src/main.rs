mod brightness;
mod clock;
mod fans;
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
        for t in &mut self.values {
            if t.0 == key {
                t.1 = value.clone();
                return;
            }
        }
    }
}

struct Poller<'a> {
    key: &'a str,
    next_fire: Instant,
    interval: Duration,
    action: fn() -> String,
}

// run is a event-loop polling mechanism for our status bar.
fn run(title: &str, mut pollers: Vec<Poller>) {
    let mut bar = Bar {
        title,
        values: pollers.iter().map(|t| (t.key, initializing())).collect(),
    };

    let mut lasts: Vec<String> =
        pollers.iter().map(|_| String::default()).collect();

    loop {
        let now = Instant::now();
        let mut updated = false;
        for (i, poller) in pollers.iter_mut().enumerate() {
            // Skip undued pollers
            if now < poller.next_fire {
                continue;
            }
            // Only apply updates if field value changed from last
            let result = (poller.action)();
            if lasts.get(i).unwrap() == result.as_str() {
                continue;
            }
            updated = true;
            lasts[i] = result.clone();
            poller.next_fire = now + poller.interval;
            bar.update(poller.key, result);
        }
        if updated {
            println!("{}", bar);
        }

        // Find the soonest next poller
        let next = pollers.iter().map(|t| t.next_fire).min().unwrap();
        // Sleep until then (no busy-waiting)
        std::thread::sleep(next - Instant::now());
    }
}

fn main() {
    let now = Instant::now();
    run(
        "dwmbar-rs",
        vec![
            Poller {
                key: "key1",
                next_fire: now,
                interval: Duration::from_secs(1),
                action: || String::from("key1 value"),
            },
            Poller {
                key: "fans",
                next_fire: now,
                interval: Duration::from_millis(500),
                action: fans::get,
            },
            Poller {
                key: "brightness",
                next_fire: now,
                interval: Duration::from_secs(3),
                action: brightness::get,
            },
            Poller {
                key: "clock",
                next_fire: now,
                interval: Duration::from_millis(500),
                action: clock::get,
            },
        ],
    );
}

fn initializing() -> String {
    String::from("initializing...")
}
