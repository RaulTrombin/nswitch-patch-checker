use std::io;

fn main() {
    println!("Enter the Switch serial number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let serial = input.trim();

    if serial.starts_with("XAW1") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        println!("{serial_number}");
        if serial_number >= 10000000000 && serial_number <= 10074000000 {
            println!("safe to buy");
        } else if serial_number > 10074000000 && serial_number <= 10120000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    } else if serial.starts_with("XAW4") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        if serial_number >= 40000000000 && serial_number <= 40011000000 {
            println!("safe to buy");
        } else if serial_number > 40011000000 && serial_number <= 40012000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    } else if serial.starts_with("XAW7") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        if serial_number >= 70000000000 && serial_number <= 70017800000 {
            println!("safe to buy");
        } else if serial_number > 70017800000 && serial_number <= 70030000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    } else if serial.starts_with("XAJ1") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        if serial_number >= 10000000000 && serial_number <= 10020000000 {
            println!("safe to buy");
        } else if serial_number > 10020000000 && serial_number <= 10030000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    } else if serial.starts_with("XAJ4") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        if serial_number >= 40000000000 && serial_number <= 40046000000 {
            println!("safe to buy");
        } else if serial_number > 40046000000 && serial_number <= 40060000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    } else if serial.starts_with("XAJ7") {
        let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
        if serial_number >= 70000000000 && serial_number <= 70040000000 {
            println!("safe to buy");
        } else if serial_number > 70040000000 && serial_number <= 70050000000 {
            println!("possibly patched");
        } else {
            println!("definitely patched");
        }
    }
}
