use concrete::prelude::*;
use concrete::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::str;

fn main() -> Result<(), Error> {
    let input = File::open("src/concrete_jungle.txt")?;
    let mut buffered = BufReader::new(input);

    let mut buffer = vec![];
    buffered.read_to_end(&mut buffer).unwrap();

    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let mut new_buffer = vec![];
    for value in &buffer {
        let new_value = FheUint8::encrypt(*value, &client_key);
        new_buffer.push(new_value);
    }

    let mut rng = rand::thread_rng();
    new_buffer.shuffle(&mut rng);

    let mut final_buffer = vec![];
    new_buffer.iter().for_each(|new_value| {
        let decrypted_value: u8 = new_value.decrypt(&client_key);
        final_buffer.push(decrypted_value);
    });

    let s = str::from_utf8(&final_buffer).unwrap();

    print!("{}", s);

    Ok(())
}
