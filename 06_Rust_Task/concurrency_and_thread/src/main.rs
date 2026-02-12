use chrono::{DateTime, Local};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::Duration;
#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    record_added_time: String,
    thread_id: String,
}

fn convert_str_to_local_time(time_str: &String) -> DateTime<Local> {
    let dt = DateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();
    dt.with_timezone(&Local)
}

fn main() {
    let shared_store = Arc::new(RwLock::new(Vec::<MultiThread>::new()));

    let primary_key = Arc::new(AtomicI32::new(0));

    // record creator
    {
        let store = Arc::clone(&shared_store);
        let counter = Arc::clone(&primary_key);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let new_id = counter.fetch_add(1, Ordering::SeqCst) + 1;

            let record = MultiThread {
                id: new_id,
                record_added_time: Local::now().to_string(),
                thread_id: rand::random::<i32>().to_string(),
            };

            {
                let mut store_wl = store.write().unwrap();
                store_wl.push(record);
            }
        });
    }

    // thread 2 // continusily read the record

    {
        let store = Arc::clone(&shared_store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let store_rl = store.read().unwrap();
            println!("{:#?}", store_rl);
        });
    }

    // thread 3  even cleaner

    {
        let store = Arc::clone(&shared_store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let mut store_wl = store.write().unwrap();
            let now = Local::now();
            store_wl.retain(|record| {
                if record.id & 1 == 0 {
                    let diff = now - convert_str_to_local_time(&record.record_added_time);
                    return diff.num_seconds() <= 20;
                }
                true
            });
        });
    }

    // thread 4 odd cleaner

    {
        let store = Arc::clone(&shared_store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let mut store_wl = store.write().unwrap();
            let now = Local::now();
            store_wl.retain(|record| {
                if record.id & 1 == 1 {
                    let diff = now - convert_str_to_local_time(&record.record_added_time);
                    return diff.num_seconds() <= 20;
                }
                true
            });
        });
    }

    // thread 5 Even Counter
    {
        let store = Arc::clone(&shared_store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let store_rl = store.read().unwrap();
            let even_count = store_rl.iter().filter(|record| record.id & 1 == 0).count();
            println!(" Even Count: {}", even_count);
        });
    }

    // thread 6 Odd Counter
    {
        let store = Arc::clone(&shared_store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let store_rl = store.read().unwrap();
            let odd_count = store_rl.iter().filter(|record| record.id & 1 == 1).count();
            println!(" Odd Count: {}", odd_count);
        });
    }

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
