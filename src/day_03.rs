use std::io;

pub fn run() {
    let mut diagnostic = Vec::<u32>::new();
    let mut size: usize = 0;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => { break; }
            Ok(_) => {
                line = line.trim().to_string();
                if size == 0 {
                    size = line.len();
                }
                println!("line: '{}'", line);
                let binary = u32::from_str_radix(&line, 2).unwrap();
                diagnostic.push(binary);
            }
            Err(err) => {
                panic!("bad line: {}", line);
            }
        }
    }

    let mut bits: Vec::<i32> = vec![0; size];
    for binary in diagnostic {
        println!("binary: {:b}", binary);
        let mut tmp = binary;
        for i in 1..(size + 1) {
            let lsb = tmp & 1;
            match lsb {
                0 => { bits[size - i] -= 1; }
                1 => { bits[size - i] += 1; }
                _ => { }
            }
            tmp >>= 1;
        }
    }
    println!("bits: {:?}", bits);
    
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in 0..size {
        gamma <<= 1;
        epsilon <<= 1;
        if bits[i] > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("gamma: {:b}\nepsilon: {:b}\nconsumption: {}", gamma, epsilon, gamma * epsilon);
}
