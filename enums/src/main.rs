
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum IpAddr2 {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));
}
pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
}