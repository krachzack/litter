use app;
use failure::Error;

pub struct Launcher;

impl Launcher {
    pub fn run() -> Result<(), Error> {
        let matches = app::new().get_matches();

        println!("{:?}", matches);

        Ok(())
    }
}
