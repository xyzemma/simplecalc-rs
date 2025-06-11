use std::io;
enum exprpartval {
    num(f32),
    oper(char),
}
struct exprpart {
    val: exprpartval,
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
        eval(input);
    }
}
fn eval(expr: String) -> f32 {
        let operators: [char;4] = ['+','-','*','/'];
        let mut exprvec: Vec<exprpart> = Vec::new();
        let mut active_num = String::new();
        let mut index: i32 = -1;
        let mut pos: i32 = 0;
        for i in expr.chars() {
            index += 1;
            if i.is_digit(10) == true{
                active_num.push(i);
            } else if i == '.' && expr.chars().nth((index-1).try_into().unwrap()).unwrap().is_digit(10) && expr.chars().nth((index+1).try_into().unwrap()).unwrap().is_digit(10){
                active_num.push(i);
            } else {
                if operators.contains(&i) {


                } else if i.is_whitespace() {
                    if active_num.is_empty() == false {
                        let anf32: f32 = active_num.clone().parse().unwrap();
                        let a_pos: i32 = pos;
                        let a_exprnum = exprpart {val: exprpartval::num(anf32), pos: a_pos};
                        exprvec.push(a_exprnum);
                        active_num = String::new();
                        pos += 1;
                    }
                }
            }
        }
        return 0.0;
}