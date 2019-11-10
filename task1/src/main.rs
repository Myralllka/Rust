fn main() {


    let mut number:i32 = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("Hello world");
    let x = 65;
    let z = {
        let x = 6;
        x + 1
    };
    println!("{}", z);
    func2(z);
    let num = 8;
    if num < 10 {
        println!("less 10")
    } else {
        println!("more 10")
    }
    let number = {
        if num < 10 {
            69
        } else {
            96
        }
    };
    println!("number {}", number);
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn func2(number: i32) {
    println!("Hello {}", number);
}