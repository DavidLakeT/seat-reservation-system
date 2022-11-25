use seat_reservation_system::client::terminal::{get_user_info, get_menu_selection};
use std::process;
use seat_reservation_system::server::user::User;

fn main() {

    let run_mode = std::env::args().nth(1);

    match run_mode {

        Some(mode) => {

            match mode.as_str() {
                "SERVER" => {}, //Inicializar el servidor y los sockets
                "CLIENT" => {

                    let user_info = get_user_info();
                    let user = User::default(user_info.0, Some(user_info.1), Some(user_info.2));
                    get_menu_selection(&user);
                }, //Inicializar el cliente y el connection
                _ => { println!("Non Valid Run mode."); process::exit(0); },
            }
        }

        None => println!("Run mode not specified. Use cargo run <mode>"),
    }
}