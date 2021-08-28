use std::net::TcpStream;
use std::str;
use std::io::{self, BufReader, Read, Write};
fn main() {
    let mut input = String::new();
    let mut datasize;
    let mut messageFromServer = String::with_capacity(512);
    let mut buf = [0;512];
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Could not conect to server");
    loop {          
        println!("Enter message");
        io::stdin().read_line(&mut input).expect("Failed to read from console input");
        stream.write(input.as_bytes()).expect("Failed to write to server");
        if input.contains("quit") {
            println!("Quiting client app");
            break;
        }
        input.clear();


        datasize = stream.read(&mut buf).unwrap();
        println!("Data read from the sever :: {}", datasize);
        messageFromServer = str::from_utf8(&buf[..]).expect("Cannot convert data from server to string").to_owned();
        println!("Message from Server :: {}",messageFromServer);
        messageFromServer.clear();


        


        
        
        // let mut reader = BufReader::new(&stream);
        // reader.read_until(b'\n', &mut buffer).expect("Could not read from buffer");
    //    println!("{}", str::from_utf8(&buffer).expect("Could not write msg as string"));

    }
}
