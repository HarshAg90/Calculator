use std:: io;

fn main() {
    println!("welcome to my calculator, enter a comand to use it");
    println!("1 - add to a problem");
    println!("2 - subtract from happiness");
    println!("3 - multiply misery");
    println!("4 - devide more in my mind");
    println!("0 - get out");


    let mut input_text = String::new();
    io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    let mut arg = 0;
    match trimmed.parse::<u32>() {
        Ok(i) => {
            arg = i;
        },
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    
    match arg{
        0 => println!("quitting"),
        1 => {
            println!("you selected addition");
            println!("enter numer and press `enter`, enter a string to quit");
            let mut sol = 0;
            loop {
                let mut inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("failed to read from stdin");
                let trim_text = inp.trim();
                match trim_text.parse::<u32>() {
                    Ok(i) => {
                        sol += i;
                    },
                    Err(..) => {
                        println!("Quiting");
                        break;
                    },
                };
            }
            println!("Sum of numbers is {}",sol);
        },
        2 => {
            println!("you selected Subtraction");
            println!("enter 1st number -> enter -> 2nd number -> enter");
            let mut a = 0;
            let mut b = 0;
            let mut inp_a = String::new();
            io::stdin()
                .read_line(&mut inp_a)
                .expect("failed to read from stdin");
            let trim_a = inp_a.trim();
            match trim_a.parse::<u32>(){
                Ok(i) => {
                    a = i;
                },
                Err(..) => {
                    println!("Quiting");
                },
            };
            let mut inp_b = String::new();
            io::stdin()
                .read_line(&mut inp_b)
                .expect("failed to read from stdin");
            let trim_b = inp_b.trim();
            match trim_b.parse::<u32>(){
                Ok(i) => {
                    b = i;
                },
                Err(..) => {
                    println!("Quiting");
                },
            };
            println!("difference of numbers is {}",a - b);
        }
        3 => {
            println!("you selected Multiplication");
            println!("enter numer and press `enter`, enter a string to quit");
            let mut sol = 1;
            loop {
                let mut inp = String::new();
                io::stdin()
                .read_line(&mut inp)
                .expect("failed to read from stdin");
                let trim_text = inp.trim();
                match trim_text.parse::<u32>() {
                    Ok(i) => {
                        sol *= i;
                    },
                    Err(..) => {
                        println!("Quiting");
                        break;
                    },
                };
            }
            println!("multiply of numbers is {}",sol);
        }
        4=>{
            println!("you selected division");
            println!("enter 1st number -> enter -> 2nd number -> enter");
            let mut a = 0;
            let mut b = 0;
            let mut inp_a = String::new();
            io::stdin()
                .read_line(&mut inp_a)
                .expect("failed to read from stdin");
            let trim_a = inp_a.trim();
            match trim_a.parse::<u32>(){
                Ok(i) => {
                    a = i;
                },
                Err(..) => {
                    println!("Quiting");
                },
            };
            let mut inp_b = String::new();
            io::stdin()
                .read_line(&mut inp_b)
                .expect("failed to read from stdin");
            let trim_b = inp_b.trim();
            match trim_b.parse::<u32>(){
                Ok(i) => {
                    b = i;
                },
                Err(..) => {
                    println!("Quiting");
                },
            };
            println!("devide of numbers is {}",a/b);
        }
        _ => println!("select correct option"),
    }
}
