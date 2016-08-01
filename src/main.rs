extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;
extern crate params;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedQuery;
/// usage : http://localhost:3000/gcd?a=12&b=5

fn main() {

    let mut router = Router::new();

    router.get("/", get_form);
    router.get("/gcd", post_gcd);

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GCD Calculator</title>
    <form>
        <input type="text" name="a"/>
        <input type="text" name="b"/>
        <button type="submit">Compute GCD</button>
    </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let mut response = Response::new();
    
    let hashmap;
    match request.get_ref::<Params>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => {
            hashmap = map;
        }
    }

    let mut unparsed_numbers = Vec::new();
    match hashmap.find(&["a"]) {
        Some(&Value::String(ref value)) => {
            unparsed_numbers.push(value);
        }
        _ => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Value for 'a' parameter nota number. \n"));
            return Ok(Response::with(iron::status::NotFound));
        }
    }
    match hashmap.find(&["b"]) {
        Some(&Value::String(ref value)) => {
            unparsed_numbers.push(value);
        }
        _ => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Value for 'b' parameter nota number. \n"));
            return Ok(Response::with(iron::status::NotFound));
        }
    }

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("Value for 'n' parameter nota number: {:?}\n", unparsed));
                return Ok(Response::with(iron::status::NotFound));
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
                             numbers,
                             d));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
