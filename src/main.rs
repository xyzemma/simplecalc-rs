fn main() {
    println!("{}",n_fibonacci(3));
    println!("{}",pythagoras(3.0,4.0));
    let (r1,r2) = quadeq(2.0,-8.0,6.0);
    println!("{r1},{r2}");
    println!("{}",is_even(29832));
}
fn n_fibonacci(x:usize) -> usize {
    let mut fibonacci = Vec::new();
    fibonacci.push(0);
    fibonacci.push(1);
    for i in 2..x+1{
        fibonacci.push(fibonacci[i-1]+fibonacci[i-2]);
    }
    fibonacci[x]
}
fn pythagoras(a:f32,b:f32) -> f32 {
    return (a.powf(2.0)+b.powf(2.0)).sqrt()
}
fn quadeq(a:f32,b:f32,c:f32) -> (f32,f32) {
    let d: f32 = b.powf(2.0)-4.0*a*c;
    if d > 0.0 {
        let r1 = (b*-1.0+d.sqrt())/(2.0*a);
        let r2 = (b*-1.0-d.sqrt())/(2.0*a);
        return (r1,r2)
    } else if d == 0.0 {
        let r1 = (b*-1.0)/(2.0*a);
        let r2 = f32::NAN;
        return (r1,r2)
    } else {
        let r1 = f32::NAN;
        let r2 = f32::NAN;
        return (r1,r2)
    }
}
fn is_even(x:i32) -> bool{
    if x % 2 == 0{
        true
    } else {
        false
    }
}
