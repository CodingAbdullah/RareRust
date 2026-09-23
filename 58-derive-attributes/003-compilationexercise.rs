#[derive(Debug)]
enum Foo {
    Foo
}

#[derive(Clone)]
enum Bar {
    Bar
}

fn main() {
    let f = Foo::Foo;
    let b = Bar::Bar;
    
    println!("{:?}", f);
    let _b_clone = b.clone();
}
