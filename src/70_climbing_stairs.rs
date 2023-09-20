fn main() {
    println!("{}", climb_stairs(7));
}

fn climb_stairs(n: i32) -> i32 {
    fn fibonacci(upto: i32) -> i32 {
        if upto == 0 || upto == 1 {
            return upto;
        }

        let mut s = 1;
        let mut l = 1;
        for _ in 1..upto {
            let temp = s + l;
            s = l;
            l = temp;
        }
        l
    }
    fibonacci(n)
}
