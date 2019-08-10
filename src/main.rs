/* 
 vim: ft=rs tw=200

Flurry JSON fixing utility. 

Since Flurry comes as multi-line file with each line being json object,
its not convinient for importing into Pandas or similar libraries.
This utility converts Flurry format into single root json object with
an array of Flurry events.

Copyright © 2019 Eugene Mihailenco <mihailencoe@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use clap::{Arg, App};

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
    let source : Box<dyn Write> = match output_filename_opt {
        Some(path) => Box::new(File::create(path).unwrap()),
        None => Box::new(io::stdout())
    };

    let mut writer = BufWriter::new(source);

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
        writer.write(b",").unwrap();
        writer.write(&buf).unwrap();
    }

    // Footer
    writer.write(b"]\n}\n").unwrap();

    Ok(())
}
