extern crate iron;

use iron::prelude::*;
use iron::status;
use std::process::Command;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::str;

// if "hostname -I" responese two, more or no Ipv4Addres, this method use.
fn set_local_addres(host_port: u16) -> SocketAddrV4 {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    SocketAddrV4::new(ip, host_port)
}

fn get_addres() -> SocketAddrV4 {
    let host_port = 8080;//GAE needs 8080 ports.

    // "hostname -I" -> get the host ip addres
    let hostname_cmd = Command::new("hostname").arg("-I").output();
    let host_addres: SocketAddrV4 = match hostname_cmd {

        Ok(res) => {
            // host ip addres -> addres
            let addres = str::from_utf8(res.stdout.as_slice())
                .map_err(|err| err.to_string())
                .and_then(|ip_str| {
                    ip_str.trim()
                        .parse::<Ipv4Addr>()
                        .map_err(|err| err.to_string())
                })
                .map(|ip| SocketAddrV4::new(ip, host_port));

            match addres {
                Ok(addres) => addres,
                Err(_) => set_local_addres(host_port),
            }
        }

        Err(_) => set_local_addres(host_port),
    };

    host_addres
}

fn main() {
    let host_addres: SocketAddrV4 = get_addres();
    println!("Server listening at {}", host_addres);
    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Min Rust Docker!!"))))
        .http(host_addres)
        .unwrap();
}
