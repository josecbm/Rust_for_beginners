fn main() {
    let rec1 = Rectangle{
        width:30,
        height:30
    };
    println!("El area del rectangulo es {} pixeles cuadrados",)
}
//refactoring tuples 
fn refactoring_tupple(){
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
struct User {
    username: String,
    email: String ,
    sign_in_count: u64,
    active: bool
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
struct Rectangle{
    width: u32,
    height: u32
}