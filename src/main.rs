use std::io::{self, Read};
use std::net::TcpListener;

fn main() -> io::Result<()> {

    //Criar uma conxão TCP vinculado a 127.0.0.1::80
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Servidor rodando em http://127.0.0.1:8080");
    

    //Processar o fluxo TCP entre um soquete local e um remoto
    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        let request = String::from_utf8_lossy(&buffer);

        let parts: Vec<&str> = request.trim().split_whitespace().collect();
        let metodo = parts[0];
        let url = parts[1];
        let protocolo = parts[2];

        println!("{} {} {}", metodo, url, protocolo);
    }

    Ok(())
}
