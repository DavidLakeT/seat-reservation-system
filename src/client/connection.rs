use std::net::TcpStream;
use std::io::{ BufReader, BufRead, Write};
use std::time;
use std::io::{ Error, ErrorKind };
use crate::config::{IP_SERVERS, CONNECTION_RETRIES};

fn try_connection() -> Result<TcpStream, std::io::Error> {

    let mut current_tries = 1;
    while current_tries <= CONNECTION_RETRIES {

        //Falta agregar que se conecte a los distintos servidores SI LOS HAY
        if let Ok(stream) = TcpStream::connect(IP_SERVERS[0]) {

            return Ok(stream);
        } else {
            let dur = time::Duration::from_millis(2000);
            std::thread::sleep(dur);
            current_tries += 1;
        }
    }

    Err(Error::new(ErrorKind::Other, "ERROR"))
}

// TO DO