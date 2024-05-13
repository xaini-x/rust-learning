pub struct Customer {
    id: i32,
    name: String,
    email: String,
}

impl Customer {
   pub fn new(id: i32, name: String, email: String) -> Self {
        Self { id, name, email }
    }

}
