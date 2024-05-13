struct Car {
    owner : String,
    year: i32,
    price : i32
}

//methods
impl Car {
    //associate function
    fn insaurance()-> i32 {
123
    }
fn new(owner:String , year:i32)-> Self{
    Self{
    owner : owner,
    year: year,
    price : 0
    }
}

fn selling_price(&self)-> i32{
    self.price + Car::insaurance()
}

    fn display_car(&self){
        println!("owner:{} ,year:{} ,price:{} ",self.owner,self.year  ,self.price);
    }
    
    fn change_price(&mut self , price : i32){
        self.price = price;
    }

    fn sell(self)-> Self {
        self
    }


}

//enum 
enum TravelType {
    car(f32),
    train(f32),
    flight(f32)
}
 

impl TravelType {
    fn travel_allowance(&self ) -> f32 {
        match self {
           
            TravelType::car(miles) => miles * 2.0,
             TravelType::train(miles) => miles * 3.0,
              TravelType::flight(miles) => miles * 4.0,
        }
    }
}


struct test {
    name : String,
    marks : Option<u32>
}


fn fetch_mark (name:String ,  student_db : &Vec<test> )-> Option<u32>{
    for m in student_db {
    if m.name == name{
return  m.marks;
    }
  
}
 return None;
}

// fn check_student(name:String , student_db : &Vec<test>) -> Result<(),String>{
//     for student in student_db {
//         if student.name == name {
//             return Ok(())
//         };
//     };
//     Err(String::from("student not found"))
// }
fn check_student(name:String , student_db : &Vec<test>) -> Result<Option<u32>,String>{
    for student in student_db {
        if student.name == name {
            return Ok(student.marks)
        };
    };
    Err(String::from("student not found"))
}
fn main() {
    println!("Hello, world!");
// let mut my_car : Car = Car{
//      owner : String::from("ABC"),
//     year: 2000,
     
//     price : 12
// };
// let mut car_name = my_car.name;
// my_car.price = 2000.0;
// car_name = String::from("qwertyuio");
// println!("{car_name}  {0}", my_car.price);
// my_car.change_price(100);
// my_car.display_car();
// let new_owner = my_car.sell();
// new_owner.display_car();
let my_new_car = Car::new("mr".to_string(), 1616);
println!("{0}",my_new_car.owner);
my_new_car.display_car();
let participant = TravelType::flight(20.0);
let val  = participant.travel_allowance();
println!("checking val {}" , val);

//enum option some/none
let test_ = vec![test{
    name : "s".to_string(),
    marks: Some(100)
},
test{
    name : "su".to_string(),
    marks: Some(20)
},
test{
    name : "sus".to_string(),
    marks: None
}
];
//result ,ok 
let student = check_student("sv".to_string(),&test_);
match student {
//     Ok(_)=>{ let res = fetch_mark("s".to_string(),&test_);
//     if let Some(grade)= res{
//     println!("student marks : {grade}")
// };
// }
  Ok(Option_grade)=>{ 
    if let Some(grade)= Option_grade{
    println!("student marks : {grade}")
};
}
Err(err_msg)=>{ println!("{err_msg}");}

}
let res = fetch_mark("s".to_string(),&test_);
match res {
   Some(grade) => println!("{grade}"),

None => println!("no data")
};


}
