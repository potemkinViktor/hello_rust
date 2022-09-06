pub mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::connect(); // super ключевое слово перемещает нас на один уровень выше корневого модуля 
        // или использовать ::client::connect(); поиск в той же папке.
    }

    // use super::client;

    // #[test]
    // fn it_works() {
    //     client::connect();
    // }
}
