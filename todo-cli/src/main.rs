use std::collections::HashMap;

struct Todo {
    //use Rust to build in HashMap to store key/value pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // we pas true as the value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurre: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation failed");

    if action == "add" {
        todo.insert(item);

        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not prsentin the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occured: {}", why),
            },
        }
    }
}
