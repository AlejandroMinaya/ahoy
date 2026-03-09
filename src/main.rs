mod display;

use std::sync::mpsc::channel;
use std::thread;
use std::{fs::File, io::BufReader, path::PathBuf};

use ahoy::Ahoy;
use cli_log::init_cli_log;
use display::{AhoyIO, native_display::NativeIO};

use clap::Parser;

use crate::display::AhoyDisplayEvents;

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn main() -> anyhow::Result<()> {
    init_cli_log!();

    let args = Args::parse();
    let file = File::open(args.program)?;
    let mut reader = BufReader::new(file);

    let mut ahoy = Ahoy::default();
    ahoy.load(&mut reader)?;

    let (vtx, vrx) = channel();
    /* The new approach is to treat the displays as the IO system. Starting the system an
     * initializing the screen are now separate methods. We need to create another channel
     * to send things from the "processor" to the IO system.
     */
    let _ = NativeIO::connect_new(vtx);
    let processor = thread::spawn(move || {
        loop {
            let _ = ahoy.process();
            if let Ok(event) = vrx.try_recv() {
                match event {
                    AhoyDisplayEvents::TurnOff => break,
                }
            }
        }
    });
    let _ = processor.join();
    Ok(())
}
