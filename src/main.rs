use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Parse args
    let matches = App::new("Flurry JSON preparing tool")
        .version("1.0")
        .author("Mihailenco Evgheni <mihailencoe@gmail.com>")
        .about("Transfers multi-line Flurry raw export into proper JSON scheme")
        .arg(Arg::with_name("INPUT")
            .help("Sets the raw Flurry export file name to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets output file to write JSON data")
            .required(false)
            .index(2))
        .get_matches();

    let input_filename = matches.value_of("INPUT").unwrap();
    let output_filename_opt = matches.value_of("OUTPUT");

    // reading
    let f = File::open(input_filename)?;
    let mut reader = BufReader::new(&f);

    // writing
    let mut writer = BufWriter::new(
        match output_filename_opt {
            Some(x) => File::open(x).unwrap(),
            None => io::stdout()
        }
    );

    // Header
    writer.write(b"{ \"data\": [\n").unwrap();

    // Allocate buffer, big enough just in case
    let mut buf = vec![];
    buf.reserve(4096);

    // Read first line as is
    reader.read_until(b'\n', &mut buf).expect("don't expect to fail");

    writer.write(&buf).unwrap();

    // Subsequent lines prefixed with ','
    loop {
        buf.clear();
        reader.read_until(b'\n', &mut buf).expect("don't expect to fail");
        if buf.len() == 0 {
            break;
        }
        writer.write(b",\n").unwrap();
        writer.write(&buf).unwrap();
    }

    // Footer
    writer.write(b"]\n}\n").unwrap();

    Ok(())
}
