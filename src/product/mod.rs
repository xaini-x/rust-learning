  use category::Category;
    pub struct Product {
        id: i32,
        name: String,
        price: f64,
        category: Category,
    }

    mod category ;
    impl Product {
        fn tax_calculate(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.tax_calculate()
        }
    }