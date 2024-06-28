pub fn basic() {
    {
        let s = String::from("Luan");
        println!("{}", s);
    }
    // println!("{}", s);
}

pub fn life_cicle() {
    let x: &String;
    {
        let y: String = String::from("Luan");
        x = &y;
        println!("{} {}", x,y);
        println!("{}", x);
    }
    // println!("{}", x);
}

pub fn transfer() {
    let num = 10;
    println!("{}", num);

    let s1 = String::from("Rust");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}

pub fn clone() {
    let s1 = String::from("Luan");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
}

pub fn copy() {
    let num1 = [1,2,3,4,5];
    let num2 = num1;
    println!("{:?} {:?}", num1, num2);
}