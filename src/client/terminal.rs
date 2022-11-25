use std::io;
use std::io::Write;
use std::process;
use crate::server::user::User;

pub enum MenuSelection {

    DisplayAllSeats,
    DisplayAvailableSeats,
    DisplayReservedSeats,
    ReserveSeat(i32),
    Checkout,
    CancelReservation(i32),
}

pub fn get_user_info() -> (String, String, i32) {

    print!("Insert Username: ");
    io::stdout().flush().unwrap();
    let username = get_input();
    print!("Insert Mail: ");
    io::stdout().flush().unwrap();
    let mail = get_input();
    print!("Insert Phone Number: ");
    io::stdout().flush().unwrap();
    let phone = get_integer_input();
    println!();

    println!("------------ YOUR ACCOUNT INFO ------------");
    println!("Username: {}", username);
    println!("Mail: {}", mail);
    println!("Phone: {}\n", phone);

    (username, mail, phone)
}

pub fn get_menu_selection(user: &User) -> MenuSelection {

    loop {

        print_menu();

        let selection = get_integer_input();

        if let Some(selection) = {

            match selection {

                1 => { Some(MenuSelection::DisplayAllSeats) },
                2 => { Some(MenuSelection::DisplayAvailableSeats) },
                3 => { Some(MenuSelection::DisplayReservedSeats) },
                4 => {

                    println!("Insert num of the seat you'd like to reserve: ");
                    Some(MenuSelection::ReserveSeat(get_integer_input()))
                },
                5 => {

                    //Se debe realizar petición para consultar las sillas reservadas por el usuario
                    println!("Insert num of the seat you'd like to cancel: ");
                    Some(MenuSelection::CancelReservation(get_integer_input()))
                },
                6 => { Some(MenuSelection::Checkout) },
                7 => { process::exit(0) },
                _ => { println!("Non-Valid Input"); None },
            }
        } {

            execute(selection, user);
        }
    }
}

fn execute(selection: MenuSelection, user: &User) {

    match selection {

        MenuSelection::DisplayAllSeats => {

            //Aquí se enviará la petición al servidor para recibir la lista de todas las sillas (y su estado)
        },
        MenuSelection::DisplayAvailableSeats => {

            //Aquí se enviará la petición al servidor para recibir la lista de sillas disponibles
        },
        MenuSelection::DisplayReservedSeats => {

            //Aquí se consultará con el servidor las sillas reservadas por el usuario
        },
        MenuSelection::ReserveSeat(num) => {

            //Aquí se enviará la petición al servidor para reservar la silla
        },
        MenuSelection::CancelReservation(num) => {

            //Aquí se enviará la petición al servidor para cancelar la reserva
        },
        MenuSelection::Checkout => {

            //Aquí se enviará la información al servidor de que se ha realizado el pago
        },
    }
}

fn print_menu() {

    println!("------------ SELECT AN OPTION ------------");
    println!("1: Display All Seats");
    println!("2: Display Available Seats");
    println!("3: Display My Reserved Seats");
    println!("4: Reserve Seat");
    println!("5: Cancel Reservation");
    println!("6: Checkout");
    println!("7: Exit");
}

fn print_user_reserved_seats(user: &User) {

    user.display_seats_info();
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    input.pop();
    input
}

fn get_integer_input() -> i32 {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    input.pop();

    if let Ok(result) = input.trim().parse::<i32>() {

        return result;
    }

    -1
}