use std::io;
struct exprnum {
    num: f32,
    pos: i32
}
fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
            }
            Err(error) => println!("error: {error}"),
        }
        let mut nums: Vec<exprnum> = Vec::new();
        let mut active_num = String::new();
        let mut index: i32 = -1;
        let mut pos: i32 = 0;
        for i in input.chars() {
            index += 1;
            if i.is_digit(10) == true{
                active_num.push(i);
            } else if i == '.' && input.chars().nth((index-1).try_into().unwrap()).unwrap().is_digit(10) && input.chars().nth((index+1).try_into().unwrap()).unwrap().is_digit(10){
                active_num.push(i);
            } else {
                if active_num.is_empty() == false {
                    let anf32: f32 = active_num.clone().parse().unwrap();
                    let a_pos: i32 = pos;
                    let a_exprnum = exprnum {num: anf32, pos: a_pos};
                    nums.push(a_exprnum);
                    active_num = String::new();
                    pos += 1;
                }
            }
        }
    }
}
