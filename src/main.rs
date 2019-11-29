mod circle_safe_ext;
use circle_safe_ext::Circle;
mod circle_safe_int;
use circle_safe_int::Circle as C;

fn main() {
    let mut circle: C<i32> = C::new();
    circle.push_after(42);
    circle.push_after(50);
    circle.push_after(49);
    circle.push_after(48);
    circle.push_after(47);
    circle.push_after(46);
    circle.push_after(45);
    circle.push_after(44);
    circle.push_after(43);
    println!("{}", circle.value().unwrap());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
    println!("{}", circle.next());
}

fn advent09(){
    let mut max: i32 = 0;
    let mut scores: [i32; 413] = [0; 413];
    let mut circle = Circle::new(0);
    for i in 0..71082{
        let v : i32 = i + 1;
        if v % 23 == 0 {
            let p : usize= (i % 413) as usize;
            scores[p] += v;
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            circle.last();
            scores[p] += circle.remove();
        }else{
            circle.next();
            circle.insert_after_step(v);
        }
    }
    
    for i in 0..413 as usize{
        if scores[i] > max{
            max = scores[i];
        }
    }
    println!("{}", max);
}
