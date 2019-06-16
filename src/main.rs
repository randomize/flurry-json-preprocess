use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let f = File::open("161545.json")?;
    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());

    // Header
    writer.write(b"{ \"data\": [\n").unwrap();

    // Read first line as is
    let mut iter = file.lines();
    let l = iter.next().unwrap().unwrap(); // result -> option
    writer.write(l.as_bytes()).unwrap();

    // Subsequent lines prefixed with ','
    for line in iter {
        writer.write(b",\n").unwrap();
        let l = line.unwrap();
        writer.write(l.as_bytes()).unwrap();
    }

    // Footer
    writer.write(b"]\n}\n").unwrap();

    Ok(())
}
