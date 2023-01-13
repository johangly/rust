fn main() {
    // dos numeros que vamos a sumar
    let mut numero :i128 = 0;
    let limit :i128 = 300000;
    loop {
        numero = numero + 1;
        println!("{}",numero);
        if numero == limit{
            break;
        }
    }
}
