use std::collections::BTreeMap;
use std::env::args;
use std::process::exit;

#[derive(Default)]
struct HistoricalDataStore {
    map: BTreeMap<String, BTreeMap<u64, String>>,
}


impl HistoricalDataStore {
    fn record(&mut self, key: &str, value: &str, time: u64) {
        self.map.entry(key.to_string()).or_insert(BTreeMap::default()).entry(time).or_insert(value.to_string());
    }

    fn retrieve(&self, key: &str, mut time: u64) -> Option<String> {
        // lookup by key
        if !self.map.contains_key(&key.to_string()) {
            return None;
        }
        // optimize with binary search
        loop {
            // lookup closest smaller version
            if let Some(v) = self.map.get(key).unwrap().get(&time) {
                return Some(v.clone());
            }
            time = time - 1;
            if time == 0 {
                return None;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <key>", args[0]);
        return;
    }
    let key = &args[1];
    let v = &args[2];
    let v = v.parse::<u64>().unwrap();

    let mut store = HistoricalDataStore::default();

    store.record("report", "initial", 2);   // save "report" with "initial" at time 2.
    store.record("report", "updated", 5);   // save "report" with "updated" at time 5.
    store.record("report", "final", 7);     // save "report" with "final" at time 7.

    // compare the key with "report" with match
    match key {
        key if key == "report" => {
            if let Some(v) = store.retrieve(key, v) { println!("{v}") } else { println!("None") }
        }
        _ => {
            eprintln!("Invalid key");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let mut store = HistoricalDataStore::default();

        store.record("report", "initial", 2);   // save "report" with "initial" at time 2.
        store.record("report", "updated", 5);   // save "report" with "updated" at time 5.
        store.record("report", "final", 7);     // save "report" with "final" at time 7.

        assert_eq!(Some("updated".to_string()), store.retrieve("report", 6));            // return "updated" (since the closest prior time is 5).
        assert_eq!(Some("initial".to_string()), store.retrieve("report", 2));            // return "initial"
        assert_eq!(Some("final".to_string()), store.retrieve("report", 8));            // return "initial"
    }

    #[test]
    fn test_fail() {
        let mut store = HistoricalDataStore::default();

        store.record("report", "initial", 2);   // save "report" with "initial" at time 2.
        store.record("report", "updated", 5);   // save "report" with "updated" at time 5.
        store.record("report", "final", 7);     // save "report" with "final" at time 7.

        assert_ne!(Some("initial".to_string()), store.retrieve("report", 6));            // return "updated" (since the closest prior time is 5).
        assert_ne!(Some("updated".to_string()), store.retrieve("report", 2));            // return "initial"
        assert_ne!(Some("initial".to_string()), store.retrieve("report", 8));            // return "initial"
    }
}