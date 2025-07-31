

# Open Redirect vulnerability 

occurs when an application redirects users to a URL that can be controlled by user input. [cite: 12] [cite\_start]Attackers exploit this to lend credibility to their phishing attacks by making it appear as if the user is visiting a trusted domain, while in reality, they are being sent to a malicious site. [cite: 14, 15] [cite\_start]This can damage user trust and the company's reputation. [cite: 16, 17]

In this lesson, we will build a demonstration application in Rust to see this vulnerability in action. We'll explore how an attacker could exploit it and then implement robust solutions to fix it.

### Demonstration Application: MusiqueAimer

We will create a simple web application for a fictional music service called "MusiqueAimer". This application will have two main redirect functionalities: one that is vulnerable to open redirection and one that is secure.

#### Project Setup

First, let's set up our Rust project.

1.  **Create a new Rust project:**

    ```bash
    cargo new open-redirect-demo
    cd open-redirect-demo
    ```

2.  **Add dependencies to `Cargo.toml`:**
    We'll need `actix-web` for our web server and `serde` for deserializing query parameters.

    ```toml
    [dependencies]
    actix-web = "4"
    serde = { version = "1.0", features = ["derive"] }
    ```

#### Application Code

Now, replace the content of `src/main.rs` with the following code. This file will contain our complete application, including the vulnerable and secure endpoints.

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
    HttpResponse::Ok().content_type("text/html").body(LOGIN_HTML)
}

// Handler to serve the fake phishing page
#[get("/phishing-site")]
async fn phishing_page() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(PHISHING_HTML)
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
```

### README.md

Here is the comprehensive `README.md` for our project.

-----

# Open Redirect Vulnerability and Mitigation in Rust with Actix-Web

This project demonstrates an Open Redirect vulnerability and its mitigation in a web application built with Rust and the `actix-web` framework.

## Lesson Summary

[cite\_start]An **Open Redirect** vulnerability occurs when an application allows user-controllable input to determine the destination of a redirect. [cite: 12] [cite\_start]An attacker can exploit this to redirect users of a legitimate website to a malicious phishing site. [cite: 13] [cite\_start]This can significantly harm the reputation of the legitimate application and lead to the compromise of user data. [cite: 16, 17]

This lesson covers:

  * **The Vulnerability**: How an attacker can craft a URL that seems legitimate but redirects to a malicious site.
  * **The Demonstration**: A hands-on example showing the vulnerability in action.
  * [cite\_start]**The Mitigation**: Implementing an "allow list" to ensure that redirects only go to safe, predefined URLs. [cite: 114]

## Application Setup

### Prerequisites

  * Rust programming language installed (`rustup`).

### Instructions

1.  **Clone the project (or create the files as described below):**
    If you are cloning a repository, skip to step 3. Otherwise, create a new project:

    ```bash
    cargo new open-redirect-demo
    cd open-redirect-demo
    ```

2.  **Update `Cargo.toml`:**
    Add the following dependencies to your `Cargo.toml` file:

    ```toml
    [dependencies]
    actix-web = "4"
    serde = { version = "1.0", features = ["derive"] }
    ```

3.  **Create `src/main.rs`:**
    Place the provided Rust code into `src/main.rs`.

4.  **Run the Application:**
    Start the server with the following command:

    ```bash
    cargo run
    ```

    The application will be running at `http://127.0.0.1:8080`.

## Interacting with the Application

### Vulnerability Demonstration

We will simulate an attacker convincing a user to click a malicious link.

1.  **The Malicious Link**:
    An attacker crafts a URL that uses the vulnerable endpoint of our legitimate application (`http://127.0.0.1:8080`) to redirect to their phishing site.

2.  **Execute the Attack**:
    Open a new terminal and use `curl` to simulate a user clicking this link. The `-L` flag tells `curl` to follow redirects.

    ```bash
    curl -L "http://127.0.0.1:8080/vulnerable_redirect?redirect_url=http://127.0.0.1:8080/phishing-site"
    ```

3.  **Observe the Result**:
    You will see the HTML content of the `/phishing-site` page. The user starts at a trusted domain but is seamlessly redirected to the attacker's page, which is designed to steal their credit card information.

    You can also paste the URL into your browser to see the redirect happen visually.

### Mitigation Demonstration

Now, we'll see how the secure endpoint prevents this attack.

1.  **Attempted Attack on the Secure Endpoint**:
    The attacker tries the same trick, but this time targeting the `/secure_redirect` endpoint.

    ```bash
    curl -L "http://127.0.0.1:8080/secure_redirect?redirect_url=http://127.0.0.1:8080/phishing-site"
    ```

2.  **Observe the Secure Result**:
    This time, instead of the phishing page, you will see the HTML content of the legitimate `/home` page. Our application checked the `redirect_url` against a list of allowed locations and, since the malicious URL was not on the list, defaulted to a safe redirect.

3.  **Legitimate Use of the Secure Endpoint**:
    To see the secure redirect working as intended, use a URL from the allow list.

    ```bash
    curl -L "http://127.0.0.1:8080/secure_redirect?redirect_url=/home"
    ```

    This command will correctly redirect you to the home page, demonstrating that the functionality is preserved for legitimate use cases.

This lesson has demonstrated the danger of Open Redirect vulnerabilities and provided a clear, effective mitigation strategy using an allow list in a Rust `actix-web` application.