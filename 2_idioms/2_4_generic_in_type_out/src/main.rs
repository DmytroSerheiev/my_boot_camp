use std::borrow::Cow;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

fn main() {
    println!("Refactor me!");

    // Створюємо новий об'єкт помилки зі стандартним кодом "NO_USER".
    let mut err = Error::new("NO_USER");
    // Встановлюємо статус помилки 404 та додаємо повідомлення.
    err.status(404).message("User not found");
}

// Структура, що представляє помилку з кодом, статусом та повідомленням.
#[derive(Debug)]
pub struct Error<'a> {
    code: Cow<'a, str>,
    status: u16,
    message: Cow<'a, str>,
}

// Реалізація для створення стандартної помилки.
impl<'a> Default for Error<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            code: Cow::Borrowed("UNKNOWN"),
            status: 500,
            message: Cow::Borrowed("Unknown error has happened."),
        }
    }
}

// Реалізація методів для структури помилки.
impl<'a> Error<'a> {
    // Конструктор для створення помилки з конкретним кодом.
    pub fn new(code: impl Into<Cow<'a, str>>) -> Self {
        Self {
            code: code.into(),
            ..Self::default()
        }
    }

    // Метод для встановлення статусу помилки.
    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    // Метод для встановлення повідомлення помилки.
    pub fn message(&mut self, m: impl Into<Cow<'a, str>>) -> &mut Self {
        self.message = m.into();
        self
    }
}

// Структура, що представляє сервер з опційним адресою та портом.
#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    // Метод для прив'язки сервера до конкретної IP-адреси та порту.
    pub fn bind(&mut self, ip: IpAddr, port: u16) {
        self.0 = Some(SocketAddr::new(ip, port))
    }
}

// Модуль для тестування сервера.
#[cfg(test)]
mod server_spec {
    use super::*;

    // Внутрішній модуль для тестування методу bind.
    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            // Прив'язуємо сервер до IPv4-адреси та порту 8080.
            server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            // Прив'язуємо сервер до IPv6-адреси та порту 9911.
            server.bind(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
