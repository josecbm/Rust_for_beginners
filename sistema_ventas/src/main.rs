use std::io;
struct Producto{
    nombre:String,
    valor:f32
}
impl Producto{
    fn new(nombre:String , valor:f32)-> Producto{
        Producto{
            nombre:nombre,
            valor:valor
        }
    }
    fn get_nombre_producto(&self) -> &String{
        &self.nombre
    }
    fn get_valor_producto(&self) -> &f32{
        &self.valor
    }
}
struct Cliente{
    nombre:String,
    nit:String
}
impl Cliente{
    fn new(nombre:String , nit:String)->Self{
        Cliente{
            nombre,
            nit
        }
    }
    fn get_nombre_cliente(&self) -> &String{
        &self.nombre
    }
    fn get_nit(&self) -> &String{
        &self.nit
    }
}

struct Compra{
    comprador: Cliente,
    producto: Producto,
    cantidad: i32
}
impl Compra{
    fn new(client:Cliente , product:Producto , cantidad:i32) -> Compra{
        Compra{
            comprador: client,
            producto: product,
            cantidad: cantidad
        }
    }
    fn get_comprador(&self) -> &Cliente{
        &self.comprador
    }
    fn get_producto(&self) -> &Producto{
        &self.producto
    }
    fn get_cantidad(&self) -> &i32{
        &self.cantidad
    }
}
fn agregar_fruta_verdura(vec: &mut Vec<Producto>){
    let mut opcion_fruta_verdura = String::new();
    loop{
        println!("ingrese el nombre de la fruta/verdura");
        let mut nombre_fruta_verdura = String::new();
        let mut precio_fruta_verdura = String::new();
        io::stdin().read_line(&mut nombre_fruta_verdura).expect("Fallo la lectura de la fruta/verdura");
        
        println!("Ingrese el precio 0.00 de la fruta/verdura");
        io::stdin().read_line(&mut precio_fruta_verdura).expect("Fallo la lectura de la fruta/verdura");
        let precio_fruta_verdura : f32 = precio_fruta_verdura.trim().parse().expect("Porfavor ingrese un numero con punto decimal");
        vec.push(Producto::new(nombre_fruta_verdura,precio_fruta_verdura));

        println!("Desea agregar otro producto 1[SI] 2[NO]");
        opcion_fruta_verdura = "".to_string();
        io::stdin().read_line(&mut opcion_fruta_verdura).expect("Fallo la lectura ");
        println!("{}",opcion_fruta_verdura);
        let opcion_fruta_verdura : u32 = opcion_fruta_verdura.trim().parse().expect("Porfavor ingrese un numero");
        if opcion_fruta_verdura == 2 {
            break
        }
    }
}

fn buying_item(vec_productos:  &Vec<Producto>,vec_cliente:  &mut Vec<Cliente>, vec_compra :  &mut Vec<Compra> ){
    let mut opcion_compra = String::new();
    let mut cantidad = String::new();
    let mut nombre_cliente = String::new();
    let mut nit_cliente = String::new();
    if vec_productos.len() > 0{
        println!("Este es nuestro catalogo de productos disponibles");
        let mut cont = 0;
        for product in  vec_productos.iter(){
            println!("{}-> {} ..... Q.{}",cont,product.get_nombre_producto(),product.get_valor_producto());
            cont += 1;
        }
        println!("Elija el numero de producto que desea llevar");
        io::stdin().read_line(&mut opcion_compra).expect("Fallo la lectura ");
        let opcion_compra : u32 = opcion_compra.trim().parse().expect("Porfavor ingrese un numero");
        println!("Usted escogio {} con un precio de {}",vec_productos[1].get_nombre_producto(),vec_productos[1].get_valor_producto());
        println!("Ingrese la cantidad de items ");
        io::stdin().read_line(&mut cantidad).expect("Fallo la lectura ");
        let cantidad : i32 = cantidad.trim().parse().expect("Porfavor ingrese un numero");
        println!("Ingrese su nombre porfavor");
        io::stdin().read_line(&mut nombre_cliente).expect("Fallo la lectura del cliente");
        println!("Ingrese su numero de nit");
        io::stdin().read_line(&mut nit_cliente).expect("fallo la lectura del nit");
        let nuevo_cliente = Cliente::new(nombre_cliente,nit_cliente);
        let a = vec_productos[1].get_nombre_producto();
        let b = vec_productos[1].get_valor_producto();
        let producto_lleva = Producto::new(a.to_string(),*b);
        vec_compra.push(Compra::new(nuevo_cliente,producto_lleva,cantidad));
    }
}

fn visualizar_ventas(vec_ventas: &Vec<Compra>){
    let mut cont = 0;
    let mut  opcion_pause = String::new();

    for venta in vec_ventas.iter(){
        
        println!("#{} Comprador: {}\n nit: {} \n producto: {} \n cantidad: {}",
        cont,
        venta.get_comprador().get_nombre_cliente(),
        venta.get_comprador().get_nit(),
        venta.get_producto().get_nombre_producto(),
        venta.get_cantidad()
        );
        cont+=1;
    }
    io::stdin().read_line(&mut opcion_pause).expect("jejej");

}
fn main() {
    let mut vec_productos:Vec<Producto> = Vec::new();
    let mut vec_cliente:Vec<Cliente> = Vec::new();
    let mut vec_compra:Vec<Compra> = Vec::new();
    loop{
        
        println!("Bienvenido a la tienda de frutas y verduras");
        println!("1. Comprar frutas y verduras");
        println!("2. Ventas ");
        println!("3. agregar fruta y verdura");
        println!("4. Salir");
        let mut opcion_menu  = String::new();
    
        io::stdin().read_line(&mut opcion_menu).expect("Fallo la lectura de la linea");
        let opcion_menu : u32 = opcion_menu.trim().parse().expect("Ingrese un numero porfavr");
        if opcion_menu == 1 {
            buying_item(&vec_productos, &mut vec_cliente , &mut vec_compra);
        }else if opcion_menu == 2 {
            visualizar_ventas(&vec_compra);
        }
        else if opcion_menu == 3{
            agregar_fruta_verdura(&mut vec_productos);
        }
        if opcion_menu == 4 {
            break;
        }    
    }
}
