fn main() {
    // 解构元组，忽略 x
    let tup = (1, 6.4, 1);
    let (_, y, z) = tup;  // 用 _ 代替 x
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 数组 a，现在打印它
    let a:[i32;5] = [1, 2, 3, 4, 5];
    println!("Array a: {:?}", a); // 使用 a
    println!("Array a[1]: {:?}", a[1]);
    let a = [3;5];
    println!("Array a: {:?}", a); // 使用 a

    // 其他代码...
    let x = 5;
    let x = x + 1;  // 遮蔽（shadowing）
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The maximum number of points is: {}", MAX_POINTS);

    another_function(5);
    let
}

fn another_function(x: i32) {
    println!("Another function!");
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}tupodian pingjing wenti
lingyujinzhan paodaima
