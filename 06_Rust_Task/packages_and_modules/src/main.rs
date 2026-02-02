mod loops_task;
mod rwlock_mutex_task;
mod serde_json_task;
mod struct_and_method;

fn main() {
    loops_task::run();
    struct_and_method::run();
    serde_json_task::user_serialize::run();
    serde_json_task::user_deserialize::run();
    rwlock_mutex_task::request_tracker_using_mutex::run();
    rwlock_mutex_task::request_tracker_using_rwlock::run();
}
