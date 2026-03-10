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

    /* The new approach is to treat the displays as the IO system. Starting the system an
     * initializing the screen are now separate methods. We need to create another channel
     * to send things from the "processor" to the IO system.
     */
    let (io_tx, io_rx) = channel();
    let (processor_tx, processor_rx) = channel();

    let mut ahoyio = NativeIO::connect_new(io_tx, processor_rx);
    let _ = ahoyio.start();

    let processor = thread::spawn(move || {
        loop {
            let _ = ahoy.process();
            if let Ok(event) = io_rx.try_recv() {
                match event {
                    AhoyDisplayEvents::TurnOff => break,
                }
            }
            processor_tx
                .send(ahoy.current_frame)
                .expect("Frame to be sent");
        }
    });
    let _ = processor.join();
    Ok(())
}
