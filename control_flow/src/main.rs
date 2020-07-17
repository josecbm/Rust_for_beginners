fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}


fn multiply_condition(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
fn ternario_like(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
// fn ternario_dos(){
//     let condition = true;
//     if condition { 5 } else { 2 };

// }
fn loop_test(){
    loop{
        println!("again!!");
    }
}

fn returning_values(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
fn while_statement(){
    let a = [1,2,3,4,5,6];
    let mut cont = 0;

    while cont < 6{
        println!("{}",a[cont]);
        cont+=1;
    }
}

fn foreach_statement(){
    let a = [10,20,30,40,50];
    let b = ["hola","como","estas", "amigo","mio"];
    for element in a.iter(){
        println!("Elemento -> {}",element);
    }

    for element2 in b.iter(){
        println!("Elemento b ->{} ",element2);
    }

}

fn count_down(){
    for number in (1..4).rev(){
        println!("{}",number);
    }
    println!("llego al final prro")
}

fn append_text(){
    let mut s = String::from("hello");
    s.push_str(", world!");// push_str() appends a literal to a string
    println!("{}",s);
}

fn clonar(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn double_return(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len)
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}