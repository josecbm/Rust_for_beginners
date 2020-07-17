fn main() {
    println!("Hello, world!");
    funcion_nueva();
    // let x = 1;
    // println!("x {}",x);
    // let x = x + 1;
    // println!("x {}",x);
    
    // let mut x  = 5;
    // x = 4;

    // println!("{}",x);

    // fibonacci(3,2);
    // let sum = funcionSuma(5,5);
    // println!("Resultado de suma: {}",sum);
    // let resta = funcionResta(4,3);
    // println!("Resultado de resta: {}", resta);
    // println!("resultado de multiplicacion {}", funcion_multiplicacion(3,3));
    // println!("Calculadora \n 1. suma \n resta ")
}

// sistema de ventas 
// sistema de contactos 
// sistema de inventarios
struct Calculadora{
    valor1:i32,
    valor2:i32,
    resultado:i32,
    operacion:Operacion
}
impl Calculadora {
    fn new(valor1: i32 , valor2: i32) -> Calculadora{
        Calculadora {
            valor1,
            valor2,
            resultado: 0,
            operacion: Operacion::Suma
        }
    }
    fn suma(self) -> i32{
        self.valor1 + self.valor2
    }
    
}

fn funcion_nueva(){
    let mut vec_respuestas: Vec<i32> = Vec::new();
    let mut tipo_operacion=Operacion::Suma;
    println!("{}",Calculadora::new(2,3).suma());
    for num in (0..20){
        match tipo_operacion {
            Operacion::Suma => {
                let var_op = 3+2;
                vec_respuestas.push(var_op);
                let var_ref = &vec_respuestas[num]+3;
                vec_respuestas.push(var_ref);
                tipo_operacion = Operacion::Resta;
            },
            Operacion::Resta =>{
                let var_op = 3-2;
                vec_respuestas.push(var_op);
                let var_ref = &vec_respuestas[num]-3;
                vec_respuestas.push(var_ref);
                tipo_operacion = Operacion::Multiplicacion;
            },
            Operacion::Multiplicacion => {
                let var_op = 3*2;
                vec_respuestas.push(var_op);
                let var_ref = &vec_respuestas[num]*3;
                vec_respuestas.push(var_ref);
                tipo_operacion = Operacion::Division;
            },
            Operacion::Division => {
                let var_op = 3/2;
                vec_respuestas.push(var_op);
                let var_ref = &vec_respuestas[num]/3;
                vec_respuestas.push(var_ref);
                tipo_operacion = Operacion::Suma;
            }

        }
        println!("{:?}",vec_respuestas);
    }
}

enum  Operacion {
    Suma,
    Resta,
    Multiplicacion,
    Division
}


// match 

fn fibonacci(posicion : i32 , print: i32) {
    let mut siguiente  = 1;
    let mut actual = 0;
    let mut temporal = 0;

    println!("{} {}",posicion , print);
}

fn funcionSuma(exp1 : i32 , exp2: i32) -> i32{
    exp1+exp2
}

fn funcion_multiplicacion(exp1: i32 , exp2: i32) -> i32{
    exp1*exp2
}
fn funcionResta(exp1 : i32, exp2: i32) -> i32{
    exp1-exp2
}
fn funcion_calculadora(exp1 : f32 , exp2 : f32 , operation : i32) -> f32{
    if operation == 1 {
        exp1 + exp2
    }else if operation == 2 {
        exp1 - exp2
    }else if operation == 3 {
        exp1 * exp2
    }else {
        exp1 / exp2
    }
}