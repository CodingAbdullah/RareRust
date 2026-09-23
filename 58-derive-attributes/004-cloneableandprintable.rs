#[derive(Debug, Clone)]
enum Foo {
    Foo,
}

fn main() {
    let foo = Foo::Foo;

    let foo_clone = foo.clone();
    println!("{:?}", foo_clone);
}
