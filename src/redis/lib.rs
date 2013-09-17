#[desc = "Redis bindings."];
#[license = "MIT"];

struct Redis;

impl Redis {
    fn set(&self, key: &str, value: &str) {
        println(fmt!("set: %? %?", key, value));
    }

    fn get(&self, key: &str) -> ~str {
        println(fmt!("get: %?", key));

        ~"yeah"
    }
}
