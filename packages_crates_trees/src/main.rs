fn main() {
    println!("Hello, world!");
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    mod modulo2{
        pub enum ap{
            Soup,
            Salad
        }
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    let order3 = back_of_house::modulo2::ap.Soup;
}