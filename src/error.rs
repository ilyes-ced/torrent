pub enum Error {
    NetworkErr(String),
}

impl Error {
    // prints error messages
    pub fn log(self) {
        println!("error")
    }
}
