use serde::Serialize;

extern "C" {
    fn helper() -> i32;
}

#[derive(Serialize)]
struct Message {
    msg: String,
}

fn main() {
    let value = serde_json::to_string_pretty(&Message {
        msg: "Hello, world!".into(),
    });

    println!("{}: {}", value.unwrap(), unsafe { helper() });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
