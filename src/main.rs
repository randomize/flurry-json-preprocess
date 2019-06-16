use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let f = File::open("161545.json")?;
    let mut file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());

    // Header
    writer.write(b"{ \"data\": [\n").unwrap();

    // Allocate buffer, big enough just in case
    let mut buf = vec![];

    // Read first line as is
    
    let mut n = file.read_until(b'\n', &mut buf).expect("Reading failed");

    writer.write(&buf).unwrap();

    // Subsequent lines prefixed with ','
    while n != 0
    {
        writer.write(b",\n").unwrap();
        n = file.read_until(b'\n', &mut buf).expect("Reading failed");
        writer.write(&buf).unwrap();
    }

    // Footer
    writer.write(b"]\n}\n").unwrap();

    Ok(())
}
