use std::time::Duration;
use crate::config::{MAX_SEATS, PAYMENT_LIMIT_TIME};

pub enum SeatStatus {

    Available,
    OnPayment,
    Reserved,
}
pub struct Seat {

    pub number: usize,
    pub status: SeatStatus,
    pub reserve_username: Option<String>,
    pub onblock_time: Duration,
}

impl Seat {

    pub fn default(number: usize, onblock_time: u64) -> Self {

        let time = Duration::from_secs(onblock_time);

        Seat {

            number,
            status: SeatStatus::Available,
            reserve_username: None,
            onblock_time: time,
        }
    }

    pub fn wait_for_payment(&mut self) {

        self.status = SeatStatus::OnPayment;
        //Aquí podría iniciar a contar el tiempo de pago límite
    }

    pub fn set_reserved(&mut self, reserve_username: String) {

        self.reserve_username = Some(reserve_username);
    }
}

pub fn create_seats() -> Vec<Seat> {

    let mut seats = Vec::with_capacity(MAX_SEATS);

    for i in 0..MAX_SEATS {

        seats.push(Seat::default(i, PAYMENT_LIMIT_TIME));
    }

    seats
}