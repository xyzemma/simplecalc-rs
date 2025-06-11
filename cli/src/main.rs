use std::io;
enum exprpartval {
    num{value:f32},
    oper{value:char},
}
struct exprpart {
    val: exprpartval,
    pos: i32
}
fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
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
                    if active_num.is_empty() == false {
                        let anf32: f32 = active_num.clone().parse().unwrap();
                        let a_pos: i32 = pos;
                        let a_exprpart = exprpart {val: exprpartval::num{value:anf32}, pos: a_pos};
                        exprvec.push(a_exprpart);
                        active_num = String::new();
                        pos += 1;
                    }
                    if expr.chars().nth((index+1).try_into().unwrap()).unwrap().is_digit(10) && (index == 0||!expr.chars().nth((index-1).try_into().unwrap()).unwrap().is_digit(10) && active_num.is_empty()){
                        active_num.push(i);
                    }
                    if !exprvec.is_empty() {
                        let a_pos: i32 = pos;
                        exprvec.push(exprpart{val: exprpartval::oper{value:i},pos: a_pos});
                        pos += 1;
                    }

                } else if i.is_whitespace() {
                    if active_num.is_empty() == false {
                        let anf32: f32 = active_num.clone().parse().unwrap();
                        let a_pos: i32 = pos;
                        let a_exprpart = exprpart {val: exprpartval::num{value:anf32}, pos: a_pos};
                        exprvec.push(a_exprpart);
                        active_num = String::new();
                        pos += 1;
                    } 
                    } else {
                        println!("Error: Illegal characters");
                        return f32::NAN; 
                    }
                
            }
        }
        for i in &exprvec {
            match i.val {
                exprpartval::num { value } => println!("Number: {}", value),
                exprpartval::oper { value } => println!("Operator: {}", value),
            }
        } 
        return 0.0;
}
fn eval_solve(expr: Vec<exprpart>) -> Vec<exprpart>{
    let mut rvec: Vec<exprpart> = Vec::new();
    // Solve multiplications and divisions first
    let mut to_remove: Vec<usize> = Vec::new();
    for (i,val) in expr.iter().enumerate() {
        match val.val {
            exprpartval::num { value } => {},
            exprpartval::oper { value } => {
                let Operator = value;
                if Operator == '*' {
                    let mut lvalue = 0.0;
                    let mut rvalue = 0.0;
                    match expr[i-1].val {
                        exprpartval::num {value} => {
                            lvalue = value;
                        },
                        exprpartval::oper {value} => {},
                    }
                    match expr[i+1].val {
                        exprpartval::num {value} => {
                            rvalue = value;
                        },
                        exprpartval::oper {value} => {},
                    }
                    let solved = lvalue*rvalue;
                    to_remove.push(i);
                    to_remove.push(i-1);
                    to_remove.push(i+1);
                }
            },
        }
    } 

    return rvec;


}