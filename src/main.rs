use std::fmt;

#[derive(fmt::Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


struct Point {
    x: f64,
    y: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {x}, y: {y}", x=self.x, y=self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}",count,v)?;
        }

        write!(f,"]")
    }
}




fn main() {
    println!("Hello, world!");
    println!("My v1 debyg {structure:?}", structure=Structure(32));
    println!("My v2 {structure}", structure=Structure(32));

    println!("My Point {point}", point=Point{x: 32.0, y: 15.20});

    let l = List(vec![1,2,3,4]);
    println!("My new list {}", l);
}
