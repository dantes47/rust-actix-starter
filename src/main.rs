// fn main() {
//     println!("Hello, world!");
// }

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
	HttpResponse::Ok().body("Hello Guys, this time!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(index))
			.route("/about", web::get().to(index2))
	})
		.bind("localhost:8088")?
		.run()
		.await
}