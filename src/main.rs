use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct RedirectParams {
    redirect_url: String,
}

// A simple HTML page for our legitimate login form
const LOGIN_HTML: &str = "
<!DOCTYPE html>
<html>
<head>
    <title>Login - MusiqueAimer</title>
</head>
<body>
    <h1>Login to MusiqueAimer</h1>
    <p>Imagine a login form here. Once you log in, you will be redirected.</p>
</body>
</html>
";

// A simple HTML page for the fake phishing site
const PHISHING_HTML: &str = "
<!DOCTYPE html>
<html>
<head>
    <title>OMG BIG SALE!!</title>
</head>
<body>
    <h1>Enter your Credit Card Details for a HUGE Discount!</h1>
    <form>
        <label for='cc'>Credit Card:</label>
        <input type='text' id='cc' name='cc'><br><br>
        <input type='submit' value='Get Discount!'>
    </form>
    <p style='color:red;'>We will now steal your details!</p>
</body>
</html>
";

// A simple HTML page for the legitimate home page
const HOME_HTML: &str = "
<!DOCTYPE html>
<html>
<head>
    <title>Home - MusiqueAimer</title>
</head>
<body>
    <h1>Welcome to MusiqueAimer!</h1>
    <p>Your legitimate user dashboard.</p>
</body>
</html>
";

/// VULNERABLE ENDPOINT
/// This endpoint takes a `redirect_url` query parameter and redirects the user
/// to that URL without any validation. This is a classic open redirect vulnerability.
#[get("/vulnerable_redirect")]
async fn vulnerable_redirect(params: web::Query<RedirectParams>) -> impl Responder {
    // No validation is performed on the redirect_url
    HttpResponse::Found()
        .append_header(("Location", params.redirect_url.clone()))
        .finish()
}

/// SECURE ENDPOINT
/// This endpoint also takes a `redirect_url`, but it validates it against an
/// allow list of safe, internal URLs.
#[get("/secure_redirect")]
async fn secure_redirect(params: web::Query<RedirectParams>) -> impl Responder {
    let allowed_redirects = ["/home", "/profile", "/settings"];

    // The provided redirect URL is checked against the allow list.
    if allowed_redirects.contains(&params.redirect_url.as_str()) {
        HttpResponse::Found()
            .append_header(("Location", params.redirect_url.clone()))
            .finish()
    } else {
        // If the URL is not in the allow list, redirect to a default safe page.
        HttpResponse::Found()
            .append_header(("Location", "/home"))
            .finish()
    }
}

// Handler to serve the login page
#[get("/login")]
async fn login_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(LOGIN_HTML)
}

// Handler to serve the fake phishing page
#[get("/phishing-site")]
async fn phishing_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(PHISHING_HTML)
}

// Handler for the legitimate home page
#[get("/home")]
async fn home_page() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(HOME_HTML)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(login_page)
            .service(phishing_page)
            .service(home_page)
            .service(vulnerable_redirect)
            .service(secure_redirect)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
