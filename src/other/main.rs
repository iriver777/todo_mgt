use core::fmt;

#[derive(Debug)]
struct Foo {
    x: i32
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Foo {{ x: {} }}", self.x)
    }
}

fn do_sth() -> Foo {
    Foo { x:33 }
}

fn get_foo(f: Foo) {
    println!("output in get_foo: {}", f);
}

fn main() {
    println!("Hello, world!");
    let c: char = 'b';
    let size = size_of::<char>();
    println!("size is {}", size);   //{}是实现了display trait
    println!("size of c is {}!", std::mem::size_of_val(&c));

    let arr1: [u32; 5] = [1,2,3,4,5];
    println!("arr1 is {:?}", arr1);

    let c2: char = c;
    print!("c2 is {}", c2);
    print!("c is now {}", c);

    let slice1: &[u32] = &arr1[1..4];
    println!("slice1 is {:?}", slice1);     //{}加?是实现了debug trait
    println!("slice1 is {:#?}", slice1);    //{}加#可以实现格式化输出

    let mut  s1: String = String::from("hello");
    let s2: String = s1;
    println!("s2 is {}", s2);
    s1 = s2.clone();
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // let f1: Foo = do_sth();
    // println!("{}",f1);

    let mut foo: Foo = Foo {x: 42};
    let f: &mut Foo = &mut foo;

    // get_foo(foo);
    f.x = 13;
    foo.x = 33;

    println!("{}", foo.x);
    // println!("{}", f.x);
    get_foo(foo);
}
