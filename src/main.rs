
use module:: {Customer , Product ,Category, Order};
// use module::{product}; // Import the product module

fn main() {
    println!("learning library");
  
   let product1 = Product::new(1, "name".to_string(), 10.0, Category::Clothing);
// println!("product details are {}", product1);
//   product1.display_product()
let customer1 = Customer::new(1, "name".to_string(), String::from("email"));
// println!("{} {} {}" , customer1.name , customer1.id , customer1.email);
let order1 = Order::new(1, product1, customer1, 2);
println!("total cost bill {}" , order1.total_bill())
}
