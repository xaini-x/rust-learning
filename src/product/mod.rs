mod category;
pub use category::Category;

pub struct Product {
    id: i32,
    name: String,
    price: f64,
    category: Category,
}

impl Product {
    pub fn new(id: i32, name: String, price: f64, category: Category) -> Self {
        Self {
            id: id,
            name: name,
            price: price,
            category: category,
        }
    }

    pub fn display_product(&self) {
        println!(
            "id : {} ,name : {} ,price : {} ,category : {:?}",
            self.id, self.name, self.price, self.category
        );
    }
    fn tax_calculate(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.tax_calculate()
    }
}
