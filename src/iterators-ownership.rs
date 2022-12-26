// fn main() {
//     let v: Vec<i8> = vec![10, 20, 30];
//     let mut iter = v.iter();

//     let v0 = iter.next();
//     println!("v0: {v0:?}");
// }

fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.iter();

    let v0: Option<&String> = iter.next();
    print!("{:?}", v);
    println!("v0: {v0:?}");
}
