use std::io;
fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
            }
            Err(error) => println!("error: {error}"),
        }
        let mut nums: Vec<String> = Vec::new();
        let mut active_num = String::new();
        let mut index: i32 = -1;
        for i in input.chars() {
            index += 1;
            if i.is_digit(10) == true{
                active_num.push(i);
            } else if i == '.' && input.chars().nth((index-1).try_into().unwrap()).unwrap().is_digit(10) && input.chars().nth((index+1).try_into().unwrap()).unwrap().is_digit(10){
                active_num.push(i);
            } else {
                if active_num.is_empty() == false {
                    nums.push(active_num.clone());
                    active_num = String::new();
                }
            }
        }
    }
}
