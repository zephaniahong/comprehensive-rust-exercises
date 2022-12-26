fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x = true;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

