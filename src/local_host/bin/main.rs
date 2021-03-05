
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{self, Read};
use std::process::Command;

static uniq_id: [u8;8] = [5,5,2,0,8,6,0,8];

fn main() {


    let (mut stream, addr) = waiting();    
    println!("Connected to dist host {:?}", addr);

    loop {
        
        let mut line = String::new();

        println!("0: Terminate connection
        1: Say Hello
        2: 
        Enter a number:");

        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        let inst = line.trim().parse::<u64>().unwrap();

        match inst{
            0 => {send(&mut inst.to_be_bytes(),&mut stream);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            break;},
            1 => send(&mut inst.to_be_bytes(),&mut stream),
            2 => {send(&mut inst.to_be_bytes(),&mut stream);
                command(&mut stream);
            },
            _ => {},
        };


    }
    
}


fn command(stream: &mut TcpStream){
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    //let len = line.len();
    
    send(line.as_bytes(), stream);

    
}

//function to recieve data
fn rec(buffer: &mut [u8], stream: &mut TcpStream){

    println!("reciving...");
    let mut array_size: [u8;4] = [0;4];
    stream.read(&mut array_size).unwrap();
    let size = usize::from_be_bytes(array_size);


    let mut data_size = 0;
    while ! match stream.read(&mut buffer[..]){
        Ok(n) if n == size  => true,
        Ok(a) => {data_size = data_size + a;
            if data_size == size{
                println!("recieved instr");
                return;
            }
            false},
        Err(_) => panic!("dist failes to read instruction"),
    }{
        

    }
} 


fn send(buffer: & [u8], stream: &mut TcpStream){
    
    println!("sending...");
    let size = buffer.len();
    stream.write(&mut size.to_be_bytes()).unwrap();
    let mut data_size = 0;
    while ! match stream.write(& buffer[..]){
        Ok(size) => true,
        Ok(a) => {data_size = data_size + a;
            if data_size == size{
                println!("sent instruction");
                return;
            }
            false},
        Err(_) => panic!("local failed to write instruction"),
    }{
        

    }
} 


fn validate(stream: &mut TcpStream) -> bool{



    let mut data: [u8;8] = [0;8]; 
  
    //check the the read
    match stream.read_exact(&mut data[..]) {
        Ok(()) =>{for i in 0..8{
            //check if 
            if data[i] != uniq_id[i]{
                return false;
            }
        }

        },
        Err(_e)=>{panic!()},
    };

    //handle !
    stream.write(&[1]).unwrap();
    return true;

}


//do more error hanlde. Make more fault tolorent
fn waiting() -> (TcpStream, SocketAddr) {
    

    //Bind a the listen to a socket, which in this case is a local socket.alloc
    //if port is not avalible return panic(handle in future)
    
    //use severalt ports, if some port is in use
    let addrs = [
    SocketAddr::from(([0, 0, 0, 0], 8080)),
    SocketAddr::from(([0, 0, 0, 0], 8081)),
    SocketAddr::from(([0, 0, 0, 0], 8082)),
    SocketAddr::from(([0, 0, 0, 0], 8083)),
    ];
    
    let listener = TcpListener::bind(&addrs[..]).unwrap();


    loop {
        //accpect is blockin(can be non blocking)
        let res = listener.accept();
        match res{
            Ok((mut sock,addr)) => {
                if validate(&mut sock){
                    return (sock, addr)
                    }

                },
            Err(_e) =>{},
        };

    }


}