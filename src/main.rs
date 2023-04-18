use std::{io};

fn main() {
    println!("Enter the Switch serial number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let serial = input.trim();

    let answer =check_serial(serial);

    println!("{answer}");
}


fn check_serial(serial :&str )->&str{
    return
        match serial.chars().take(4).collect::<String>().as_str() {
            "XAW1" => check_xaw1(serial),
            "XAW4" => check_xaw4(serial),
            "XAW7" => check_xaw7(serial),
            "XAJ1" => check_xaj1(serial),
            "XAJ4" => check_xaj4(serial),
            "XAJ7" => check_xaj7(serial),
            "XKW1" => check_x(serial),
            "XKJ1" => check_x(serial),   
            "XJW1" => check_x(serial),
            "XWW1" => check_x(serial),
            __ => "invalid",
        }
}


    
fn check_xaw1(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 10000000000 && serial_number <= 10120000000 {
        if serial_number <= 10074000000 {
            return "safe"
        } else if serial_number <= 10120000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}

fn check_xaw4(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 40000000000 && serial_number <= 40012000000 {
        if serial_number <= 40011000000 {
            return "safe"
        } else if serial_number <= 40012000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}

fn check_xaw7(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 70000000000 && serial_number <= 70030000000 {
        if serial_number <= 70017800000 {
            return "safe"
        } else if serial_number <= 70030000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}

fn check_xaj1(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 10000000000 && serial_number <= 10030000000 {
        if serial_number <= 10020000000 {
            return "safe"
        } else if serial_number <= 10030000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}

fn check_xaj4(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 40000000000 && serial_number <= 40060000000 {
        if serial_number <= 40011000000 {
            return "safe"
        } else if serial_number <= 40012000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}


fn check_xaj7(serial: &str)  -> &str {
    let serial_number = serial[3..].parse::<i64>().unwrap_or(-1);
    if serial_number >= 70000000000 && serial_number <= 70050000000 {
        if serial_number <= 70040000000 {
            return "safe"
        } else if serial_number <= 70050000000 {
            return "maybe"
        } else {
            return "no"
        }
    } else {
        return "invalid"
    }
}

fn check_x(_serial: &str) -> &str {
    return "no";
}

