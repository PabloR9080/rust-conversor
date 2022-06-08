use std::io;

fn main() {
    println!("Conversor de grados!!");
   loop{
        println!("Ingrese 1 para convertir F a C");
        println!("Ingrese 2 para connvertir C a F");
        let mut opcion = String::new();
        io::stdin()
        .read_line(&mut opcion)
        .expect("Fallo al leer la linea");
        
        let mut grados= String::new();

        let opcion: i32 = opcion.trim().parse().expect("Se esperaba un numero");
    
        if opcion == 1{
            println!("Ingrese los grados farenheit");
            io::stdin()
                .read_line(&mut grados)
                .expect("Fallo al leer la linea");
            let grados: i32 = grados.trim().parse().expect("Se esperaba un numero");
            println!("Los {} F son {} C", grados, centigrados(grados));
        }else if opcion == 2 {
            println!("Ingrese los grados centigrados");
            io::stdin()
                .read_line(&mut grados)
                .expect("Fallo al leer la linea");
            let grados: i32 = grados.trim().parse().expect("Se esperaba un numero");
            println!("Ingrese los grados centigrados");
            println!("Los {} C son {} F", grados, farenheit(grados));
        }else{
            println!("Opcion invalida");
            break;
        } 
    }
}


fn farenheit(grados: i32) -> i32{
    (grados * 9/5)+32 
}

fn centigrados(grados: i32) -> i32{
    (grados-32)*5/9
}
