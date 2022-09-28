use patternscan::scan;
use std::io::{Write, Seek};
use std::fs::File;

fn main() {
    let pattern = concat!(
        "50 69 78 65 6C 20 41 72 74 20 42 75 66 66 65 72 ", //Pixel Art Buffer
        "? ? ? ? ",                                         //640
        "? ? ? ?"                                           //360
    );

    let mut assets = File::options()
        .read(true).write(true)
        .open("FP2_Data/sharedassets0.assets").unwrap_or_else(|_| {
        eprintln!("Unable to find sharedassets0.assets!");
        std::process::exit(1);
    });
    
    assets.seek(std::io::SeekFrom::Start(0x0 as u64)).unwrap();
    let locs = scan(&assets, pattern).unwrap_or_else(|_| {
        eprintln!("Pattern not found in file!");
        std::process::exit(1);
    });

    println!("Found loc at {:x}", locs[0]);

    print!("Type desired resolution scale [1-12]: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
 
    std::io::stdin().read_line(&mut input).unwrap();
    let multiplier = input.trim().parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid multiplier!");
        drop(input);
        std::process::exit(1);
    });

    if multiplier <= 0 || multiplier > 12 {
        eprintln!("Invalid multiplier!");
        std::process::exit(1);
    }

    let res_w = u32::to_le_bytes(640 * multiplier);
    let res_h = u32::to_le_bytes(360 * multiplier);

    assets.seek(std::io::SeekFrom::Start(0x10 + locs[0] as u64)).unwrap();
    assets.write(&res_w).unwrap_or_else(|_| {
        eprintln!("Unable to write width!");
        std::process::exit(1);
    });

    assets.write(&res_h).unwrap_or_else(|_| {
        eprintln!("Unable to write height!");
        std::process::exit(1);
    });
}