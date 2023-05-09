

fn f1(s1: String){
    let mut s2 = s1 + " K ASE";
    s2 = s2 + "_2";
    println!("{}", s2);
}


fn f2(s1: &String){
    let mut s2 = String::new();
    s2.push_str(s1);
    s2.push_str(" K ASE");
    s2.push_str("_3");
    println!("{}", s2);
}

fn f3(s1: &mut String){
    s1.push_str(" K ASE");
    println!("{}", s1);
}

fn string_uses() {
    // Hay dos tipos de str en rust: &str y String
    // &str (sring slice) es un puntero a direccion de memoria (heap o stack) 
    //    con una secuencia de chars UTF8 (guarda la longitud).
    //    No tenemos la propiedad por lo que siempre es inmutable
    // String es un objeto que hace referencia a una string en heap (guarda la longitud y la capacidad). 
    //    Tenemos la propiedad por lo que se puede hacer mutable
    let s_1:&str = "s_1";  // Esto ultimo es un s literal
    let s_2:String = String::from("s_2");
    let s_3:String = "s_3".to_string();
    let s_4:String = "s_4".to_owned();
    let s_5:&str = &s_4[0..2];
    
    let s_6 = s_2 + &s_3;  // Aqui s_6 se queda con la propiedad de s_2 por lo que no es usable ya

    // Se puede hacer una copia sin que se pierda la propiedad pero de manera menos eficiente (por copiar)
    let s_7 = format!("{}-{}-{}", s_1, s_3, s_4);
    
    // Otras maneras de concatenar son usar .concat() o concat! (esto ultimo para consevar que sea un slice= &str)
    let s_8 = ["HOLA", " ", "MUNDO"].concat();
    let s_9 = concat!("HOLA", " ", "MUNDO");

    println!("s_1: {}", s_1);
    println!("s_5: {}", s_5);
    println!("s_6: {}", s_6);
    println!("s_7: {}", s_7);
    println!("s_8: {}", s_8);
    println!("s_9: {}", s_9);
    
    // No se puede obtener el acceso a una &str mediante un indice -> let s = s_1[0]; let s = s_1&[0] // No validos
    // Pero si con rangos (hay que pasar &s_1 porque no tenemos la propiedad de s_1)
    let s_10 = &s_1[0..1];
    println!("s_10: {}", s_10);
    
    // Puede haber problemas con ciertos UTF8 chars de mayor tamaño como los emojis
    // let s = &"🦀🦀"[0..3]; //Panic: 'byte index 3 is not a char boundary; it is inside '🦀'
    // let s = &"ñ"[0..3]; //Panic: 'byte index 3 is not a char boundary; it is inside 'ñ'
    // Las letras normales tiene 1 byte mientras que las especiales tienen 4
    let s_11 = &"🦀🦀"[0..4];
    println!("s_11: {}", s_11);
    
    println!("\n");

    // Se puede hacer un bucle por los bytes con .bytes() o los chars con .chars()
    for b in "🦀🦀sí!".bytes(){
        println!("{}", b);
    }
    println!("\n");
    for c in "🦀🦀sí!".chars(){
        println!("{}", c);
    }
    println!("\n");


    
}

fn functions_uses() {
    // String inmutable
    let s1: String = "HOLA".to_string();
    // Referencia inmutable porque en la funcion no se modifica la String original
    f1(s1);
    // En este caso no podriamos imprimir de nuevo la String porque se ha prestado a la funcion
    // println!("{}", s2);

    // String inmutable
    let s2: String = "HOLA DE NUEVO".to_string();
    // Referencia inmutable porque en la funcion no se modifica la String original
    f2(&s2);
    // En este caso se pasa una referencia por lo que no se pierde la propiedad
    println!("{}", s2);

    // String mutable
    let mut s3: String = "ADIOS".to_string();
    // Referencia mutable porque en la funcion se modifica la String original
    f3(&mut s3);
    // Pero al pasarlo como referencia se modifica la s3
    println!("{}", s3);

}

fn main(){
    string_uses();
}

