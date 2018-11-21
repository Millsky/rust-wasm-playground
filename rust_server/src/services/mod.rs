use hyper::{Body, Response, Request};

fn create_body(a: i32, b: i32) -> String {
    // The || notation creates a functional closure
    // closures can capture values from the scope in which they’re called.
    let add = |x: &i32,y: &i32| -> i32 {
        return x + y;
    };
    let num = add(&a,&b);
    return format!("Number is: {}", num);
}

pub fn hello_world(_req: Request<Body>) -> Response<Body> {
    // Declare a borrowable string:
    // https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html
    Response::new(Body::from(create_body(1,2)))
}
