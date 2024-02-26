fn greet_world() {
    let chinese = "你好，世界";
    let english = "hello, world";
    let text_list = [chinese, english];
    for i in text_list {
        println!("{}", &i)
    }
}

fn main()  {
    greet_world();
}