use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref GET_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref POST_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
    static ref DELETE_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
}
enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

fn handle_request(req: Request) -> String {
    {
        let mut count = REQUEST_COUNT.write().unwrap();
        *count += 1;
    }

    match req {
        Request::Get { endpoint } => {
            {
                let mut get_count = GET_REQUEST_COUNT.write().unwrap();
                *get_count += 1;
            }

            format!("Handling GET request to {}", endpoint)
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            {
                let mut post_count = POST_REQUEST_COUNT.write().unwrap();
                *post_count += 1;
            }

            format!(
                "Handling POST request to {} with payload size {}",
                endpoint, payload_size
            )
        }
        Request::Delete(id) => {
            {
                let mut delete_count = DELETE_REQUEST_COUNT.write().unwrap();
                *delete_count += 1;
            }

            format!("Handling DELETE request for id {}", id)
        }
    }
}

fn main() {
    let get_request = Request::Get {
        endpoint: String::from("/api/data"),
    };
    println!("{}", handle_request(get_request));

    let post_request = Request::Post {
        endpoint: String::from("/api/upload"),
        payload_size: 1024,
    };
    println!("{}", handle_request(post_request));

    let delete_post = Request::Delete(1010);

    println!("{}", handle_request(delete_post));

    let total = *REQUEST_COUNT.read().unwrap();
    println!("Total requests processed: {}", total);

}
