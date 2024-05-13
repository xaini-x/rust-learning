use crate::product::Product;
use crate::customer::Customer;
    struct Order {
    id: i32,
        product: Product,
            customer: Customer,
                quantity: i32,
    }
    impl Order {
        fn calculate_discount(& self) -> f64 {
        if self.quantity > 5 {
            0.1
        } else {
            0.0
        }
    }

        fn total_bill(& self) -> f64 {
        let discount = self.calculate_discount();
        let before_discount = self.product.product_price() * self.quantity as f64;
        before_discount - (before_discount * discount)
    }
}