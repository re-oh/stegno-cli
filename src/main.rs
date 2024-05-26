use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::{arg, Command};
use png::{Decoder, Encoder, BitDepth, ColorType};

fn encode_data_into_png(image_path: &PathBuf, data_path: &PathBuf) -> io::Result<()> {

    let mut image_file = File::open(image_path)?;


    let decoder = Decoder::new(&mut image_file);
    let mut reader = decoder.read_info()?;
    let mut image_data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut image_data)?;


    let mut data_file = File::open(data_path)?;


    let mut data = Vec::new();
    data_file.read_to_end(&mut data)?;


    if data.len() * 8 > image_data.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data too large to encode in the image"));
    }

    for (i, byte) in data.iter().enumerate() {
        for bit in 0..8 {
            let image_index = i * 8 + bit;
            image_data[image_index] = (image_data[image_index] & !1) | ((byte >> (7 - bit)) & 1);
        }
    }


    let file = File::create(image_path)?;
    let ref mut w = io::BufWriter::new(file);

    let mut encoder = Encoder::new(w, reader.info().width, reader.info().height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&image_data)?;

    Ok(())
}

fn decode_data_from_png(image_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
 
    let mut image_file = File::open(image_path)?;


    let decoder = Decoder::new(&mut image_file);
    let mut reader = decoder.read_info()?;
    let mut image_data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut image_data)?;


    let mut data = Vec::new();
    let mut byte = 0;
    for (i, &pixel) in image_data.iter().enumerate() {
        byte = (byte << 1) | (pixel & 1);
        if i % 8 == 7 {
            data.push(byte);
            byte = 0;
        }
    }


    let mut output_file = File::create(output_path)?;
    output_file.write_all(&data)?;

    Ok(())
}

fn main() {
    let matches = Command::new("Stegno")
        .version("1.0")
        .about("Encodes and decodes data into and from PNG images using the Least Significant Bit method")
        .subcommand(
            Command::new("encode")
                .about("Encodes data from the specified file into the specified PNG image\nEXAMPLE: cargo run -- encode -e path/to/image.png -f path/to/data.txt")
                .arg(arg!(-e --encoder <ENCODER> "Sets the .png file that the data will be encoded in").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!(-f --file <FILE> "Sets the file that will be encoded in the png").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("decode")
                .about("Decodes data from the specified PNG image into a new file\nEXAMPLE: cargo run -- decode -e path/to/image.png -o path/to/output.txt")
                .arg(arg!(-e --encoder <ENCODER> "Sets the .png file that contains the encoded data").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!(-o --output <OUTPUT> "Sets the output file that will contain the decoded data").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encode", sub_m)) => {
            let image_path = sub_m.get_one::<PathBuf>("encoder").expect("required");
            let data_path = sub_m.get_one::<PathBuf>("file").expect("required");
            encode_data_into_png(image_path, data_path).unwrap();
        }
        Some(("decode", sub_m)) => {
            let image_path = sub_m.get_one::<PathBuf>("encoder").expect("required");
            let output_path = sub_m.get_one::<PathBuf>("output").expect("required");
            decode_data_from_png(image_path, output_path).unwrap();
        }
        _ => eprintln!("Invalid command"),
    }
}
