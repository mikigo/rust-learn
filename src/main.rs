// fn greet_world() {
//     let chinese = "你好，世界";
//     let english = "hello, world";
//     let text_list = [chinese, english];
//     for i in text_list {
//         println!("{}", &i)
//     }
// }

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    x + y
}

fn main()  {
    // greet_world();

    // let x = '中';
    // println!("{}",std::mem::size_of_val(&x));

    // let f: bool = true;
    // if f {
    //     println!("111111")
    // }
    
    println!("{}",add_with_extra(2, 3));

}