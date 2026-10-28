struct Laptop {
    brand:String,
    quantity:u32,
    price:u32
}
impl Laptop {
    fn cost_of_3(&self)->u32 {
        self.price * 3
    }
}
fn main() {
   let hp = Laptop {
    brand:String::from("Hp"),
    quantity:10,
    price:650_000
   };
   let ibm = Laptop {
    brand:String::from("IBM"),
    quantity:6,
    price:755_000
   };
   let toshiba = Laptop {
    brand:String::from("Toshiba"),
    quantity:10,
    price:550_000
   };
   let dell = Laptop {
    brand:String::from("Dell"),
    quantity:4,
    price:850_000
   };
   println!("The price total cost of 3 laptops of each brand is {}",hp.cost_of_3() + ibm.cost_of_3() + toshiba.cost_of_3() + dell.cost_of_3());
}
