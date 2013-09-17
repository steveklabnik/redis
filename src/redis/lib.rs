#[desc = "Redis bindings."];
#[license = "MIT"];

struct Redis;

// TODO: These types are all wrong, but they are all the types
enum Reply {
    Status(~str),
    Error(~str),
    Integer(int),
    Bulk(~str),
    MultiBulk(~str),
}


impl Redis {
    fn set(&self, key: &str, value: &str) {
        println(fmt!("set: %? %?", key, value));
    }

    fn get(&self, key: &str) -> ~str {
        println(fmt!("get: %?", key));

        ~"yeah"
    }
}
