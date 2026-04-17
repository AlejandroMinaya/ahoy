mod display;

use std::sync::mpsc::channel;
use std::thread::{self, sleep};
use std::time::Duration;
use std::{fs::File, io::BufReader, path::PathBuf};

use ahoy::Ahoy;
use display::{AhoyIO, native_display::NativeIO};

use clap::Parser;

use crate::display::{AhoyInputEvent, AhoyOutputEvent};

const FPS_LIMIT: u64 = 60;
const MS_PER_FRAME: Duration = Duration::from_millis(1_000 / FPS_LIMIT);

#[derive(Parser)]
struct Args {
    #[arg()]
    program: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file = File::open(args.program)?;
    let mut reader = BufReader::new(file);

    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();

    let mut ahoyio = NativeIO::connect_new(input_tx, output_rx);

    let mut ahoy = Ahoy::default();
    ahoy.load(&mut reader)?;

    let processor = thread::spawn(move || {
        loop {
            let _ = ahoy.process();
            if let Ok(event) = input_rx.try_recv() {
                match event {
                    AhoyInputEvent::TurnOff => break,
                }
            }
            output_tx
                .send(AhoyOutputEvent::NewFrame(ahoy.current_frame))
                .expect("Frame to be sent");

            sleep(MS_PER_FRAME);
        }
    });
    let _ = ahoyio.start();

    let _ = processor.join();
    Ok(())
}
