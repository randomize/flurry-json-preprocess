use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let f = File::open("161545.json")?;
    let mut reader = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());

    // Header
    writer.write(b"{ \"data\": [\n").unwrap();

    // Allocate buffer, big enough just in case
    let mut buf = vec![];

    // Read first line as is
    reader.read_until(b'\n', &mut buf).expect("don't expect to fail");

    writer.write(&buf).unwrap();

    // Subsequent lines prefixed with ','
    loop
    {
        writer.write(b",\n").unwrap();
        reader.read_until(b'\n', &mut buf).expect("don't expect to fail");
        if buf.len() == 0 {
            break;
        }
        writer.write(&buf).unwrap();
        buf.clear();
    }

    // Footer
    writer.write(b"]\n}\n").unwrap();

    Ok(())
}
