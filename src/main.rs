use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use tera::{Tera, Context};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    email: String,
    address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

impl Person {
    fn new(name: String, age: u8, email: String, address: Address) -> Self {
        Person {
            name,
            age,
            email,
            address,
        }
    }

    fn get_info(&self) -> String {
        format!("{} (Age: {})\nEmail: {}", self.name, self.age, self.email)
    }

    fn get_address(&self) -> String {
        format!(
            "{}\n{}, {}",
            self.address.street, self.address.city, self.address.zip_code
        )
    }
}

#[derive(Debug, Deserialize)]
struct PersonForm {
    name: String,
    age: String,
    email: String,
    street: String,
    city: String,
    zip_code: String,
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("title", "Person Information Form");
    let rendered = tmpl.render("index.html", &ctx).map_err(|e| {
        eprintln!("Template error: {}", e);
        actix_web::error::ErrorInternalServerError("Template error")
    })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn submit(
    tmpl: web::Data<tera::Tera>,
    form: web::Form<PersonForm>
) -> Result<HttpResponse> {
    let address = Address {
        street: form.street.clone(),
        city: form.city.clone(),
        zip_code: form.zip_code.clone(),
    };

    let person = Person::new(
        form.name.clone(),
        form.age.parse().unwrap_or(0),
        form.email.clone(),
        address,
    );

    let mut ctx = Context::new();
    ctx.insert("title", "Person Information");
    ctx.insert("person", &person);
    ctx.insert("info", &person.get_info());
    ctx.insert("address", &person.get_address());

    // Debugging: Print context data
    println!("Context data: {:?}", ctx);

    // Use the shared template instance instead of creating a new one
    let rendered = tmpl.render("result.html", &ctx).map_err(|e| {
        eprintln!("Template error: {}", e);
        actix_web::error::ErrorInternalServerError("Template error")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://localhost:8080");

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        
        App::new()
            .app_data(web::Data::new(tera))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



