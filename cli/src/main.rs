use std::io;
enum exprpartval {
    num{value:f32},
    oper{value:char},
}
struct exprpart {
    val: exprpartval,
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
                        let a_exprpart = exprpart {val: exprpartval::num{value:anf32}};
                        exprvec.push(a_exprpart);
                        active_num = String::new();
                    }
                    if expr.chars().nth((index+1).try_into().unwrap()).unwrap().is_digit(10) && (index == 0||!expr.chars().nth((index-1).try_into().unwrap()).unwrap().is_digit(10) && active_num.is_empty()){
                        active_num.push(i);
                    }
                    if !exprvec.is_empty() {
                        exprvec.push(exprpart{val: exprpartval::oper{value:i}});
                    }

                } else if i.is_whitespace() {
                    if active_num.is_empty() == false {
                        let anf32: f32 = active_num.clone().parse().unwrap();
                        let a_exprpart = exprpart {val: exprpartval::num{value:anf32}};
                        exprvec.push(a_exprpart);
                        active_num = String::new();
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
        eval_solve(exprvec); 
        return 0.0;
}
fn eval_solve(mut expr: Vec<exprpart>) -> Vec<exprpart>{
    let mut rvec: Vec<exprpart> = Vec::new();
    // Solve multiplications and divisions first
    let mut found_m_d = true;
    while found_m_d {
        found_m_d = false;
        for (i,val) in expr.iter().enumerate() {
            match val.val {
                exprpartval::num { value } => {},
                exprpartval::oper { value } => {
                    let Operator = value;
                    if Operator == '*' {
                        found_m_d = true;
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
                        expr[i] = exprpart {val: exprpartval::num{value:solved}};
                        expr.remove(i-1);
                        expr.remove(i+1);
                        break;

                    } else if Operator == '/' {
                        found_m_d = true;
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
                        let solved = lvalue/rvalue;
                        expr[i] = exprpart {val: exprpartval::num{value:solved}};
                        expr.remove(i-1);
                        expr.remove(i+1);
                        break;

                    }
                },
        }
    }
    }
    // Solving addition and subtraction next
    let mut found_a_s = true;
    while found_a_s {
        found_a_s = false;
        for (i,val) in expr.iter().enumerate() {
            match val.val {
                exprpartval::num { value } => {},
                exprpartval::oper { value } => {
                    let Operator = value;
                    if Operator == '+' {
                        found_a_s = true;
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
                        let solved = lvalue+rvalue;
                        expr[i] = exprpart {val: exprpartval::num{value:solved}};
                        expr.remove(i-1);
                        expr.remove(i+1);
                        break;

                    } else if Operator == '-' {
                        found_a_s = true;
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
                        let solved = lvalue-rvalue;
                        expr[i] = exprpart {val: exprpartval::num{value:solved}};
                        expr.remove(i-1);
                        expr.remove(i+1);
                        break;

                    }
                },
        }
    }
    }

    return rvec;


}