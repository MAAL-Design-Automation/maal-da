#[macro_use]
extern crate rocket;
use rocket::{
    fs::{relative, FileServer, Options},
    http::{Cookie, CookieJar},
    response::Redirect,
};
use rocket_dyn_templates::{context, Template};

use pulldown_cmark::{html, Parser};

fn render_markdown(file_path: &str) -> String {
    let raw_md = std::fs::read_to_string(file_path).unwrap_or_default();
    let parser = Parser::new(&raw_md);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[get("/")]
async fn index(cookies: &CookieJar<'_>) -> Template {
    let intro_text = render_markdown("content/intro.md");
    // Get theme from cookie (default to "dark")
    let theme = cookies.get("theme").map(|c| c.value()).unwrap_or("dark");

    Template::render(
        "index",
        context! {
            theme: theme,
            page_title: "MAAL | Design Automation",
            introduction: intro_text
        },
    )
}

#[post("/toggle")]
fn toggle_theme(cookies: &CookieJar<'_>) -> Redirect {
    let current = cookies.get("theme").map(|c| c.value()).unwrap_or("dark");
    let new_theme = if current == "light" { "dark" } else { "light" };
    cookies.add(Cookie::new("theme", new_theme));
    Redirect::to(uri!(index))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // add templating system
        .attach(Template::fairing())
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![index, toggle_theme])
}
