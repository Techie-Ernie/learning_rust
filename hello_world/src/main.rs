
fn main() {
    let x: i32; 
    x = 42;
    let y: i32 = 42;
    let pair = ('a', 17);
    let pair2: (char, i32) = ('b', 17);
    {
        let x = "in";
        println!("{}", x);
    }
    {
        let x = "ernie".len();
        println!("{}", x);
        let x = str::len("ernie");
        println!("{}", x);
    }
    struct Number {
        odd: bool,
        value: i32,
    }
    let z = Number{odd:False, value: 2};
}

fn test() -> i32 {
    4
}

fn test2() -> i32 {
    let feeling_lucky = true;
    match feeling_lucky {
        true => 6,
        false => 4, 
    }
}

// crate::file::function
