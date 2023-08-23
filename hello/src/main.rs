#[derive(Debug)]
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    //编译时大小未知 又想在确切大小上下文中去使用
    let list = Box::new(List::Cons(
        0,
        Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))),
    ));

    println!("{:?}", list);
}