use std::fs::File;
use image::GenericImageView;
use crate::Disk::*;
use std::path::Path;
use std::io::BufWriter;
#[path = "src/disk/disk.rs"] use Disk;

pub fn encode(disk: &Disk) -> Vec<u8> {
    let encode_aux = bincode::serialize(disk).unwrap();
    return encode_aux;
}

pub fn decode(disk: Vec<u8>) -> Disk {
    let decode = bincode::deserialize(&disk[..]).unwrap();
    return decode;
}

pub fn validate_path_FS(path: String) -> bool {
    let mut counter = 0;
    let mut result_path = String::new();
    loop { result_path = format!("{}{}{}{}", path, "/file", counter, ".png");
        if Path::new(result_path.as_str()).exists() {counter = counter + 1;} else {break;}
    } if counter == 0 { return false;}
    return true
}

pub fn validate_real_path(path: String) -> bool {
    let img = image::open(path);
    return match img { Ok(_img) => { true }, Err(_img) => { false } }
}

pub fn load_disk(path: String) -> Option<Disk> {
    let mut data: Vec<u8> = Vec::new(); let mut counter_file = 0;
    let mut path_final = format!("{}{}{}{}", path, "/file", counter_file, ".png");
    while validate_real_path(path_final.clone()) { counter_file = counter_file + 1;
        let image = image::open(path_final.clone()).unwrap();
        for pixel in image.pixels() { let pixel = pixel.2; data.push(pixel[0]); }
        path_final = format!("{}{}{}{}", path, "/file", counter_file, ".png");
    } let disk_load = decode(data); 
    return Some(disk_load);
}

pub fn write_pixels(width: u32, height: u32,mut data: Vec<u8>, save_path: &str, file_counter: u32, mut data_position: usize) {
    if data.len() < ((width * height) * (file_counter + 1)) as usize { while data.len() < ((width * height) * (file_counter + 1)) as usize { data.push(0); } }
    if data_position >= data.len() { return; }
    let final_path = format!("{}{}{}{}", save_path, "/file", file_counter, ".png");
    let path = Path::new(final_path.as_str());
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);
    let encoder = png::Encoder::new(w, width, height);
    let mut counter = 0;
    let mut pixels_colors = Vec::new();
    for i in data_position..data.len() + 1 {
        if counter == (width * height) as usize || i == data.len() {
            data_position = i;
            break;
        } else { pixels_colors.push(data[i]); counter = counter + 1; } }
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels_colors).unwrap();
    if data_position >= data.len() { return; } 
    write_pixels(width, height, data.clone(), save_path, file_counter + 1, data_position);
}