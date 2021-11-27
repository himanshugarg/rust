use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
  let server = HttpServer::new(|| {
    App::new().route("/", web::get().to(get_index))
      .route("/gcd", web::post().to(post_gcd))
  });

  println!("Serving on http://localhost:8000...");
  server.bind("127.0.0.1:8000").expect("error binding server to add")
    .run().expect("error running server")
}

fn get_index() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(
      r#"
         <title>GCD Calculator</title>
         <form action="/gcd" method="post">
           <input type="text" name="n"/>
           <input type="text" name="m"/>
           <button type="submit">Compute GCD</button>
         </form>
      "#,
  )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters {
  m: u64,
  n: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
  if form.m == 0 || form.n == 0 {
    return HttpResponse::BadRequest() 
      .content_type("text/html")
      .body("GCD with 0");
  }

  HttpResponse::Ok()
    .content_type("text/html")
    .body(format!("GCD of {} and {} is <b>{}</b>",
      form.m, form.n, gcd(form.m, form.n)))
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
