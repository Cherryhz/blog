#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate tera;

use rocket_contrib::templates::Template; 
use rocket_contrib::serve::StaticFiles;
use tera::Context;

struct Post {
    title: String,
    subtitle: String,
}

/*
fn post_list() -> Vec<Post> {
    let p1 = Post { title: "Spring is coming...".to_string(), subtitle: "Recommended activities include visits to zoos and gardens".to_string() };
    let p2 = Post { title: "菠萝包".to_string(), subtitle: "最爱的早餐".to_string()};
    vec![p1, p2]
}
*/

fn build_post(post: &Post) -> Context {
    let mut context = Context::new();
    context.insert("title", &post.title);
    context.insert("subtitle", &post.subtitle);
    context
}

/*
fn build_list(posts: Vec<Post>) -> Vec<Context> {
    let mut post_list = Vec::new();
    for p in posts {
        post_list.push(build_post(p))
    }
    post_list
}
*/

#[get("/")]
fn root() -> Template {
    let p1 = Post { 
    title: "Spring is coming...".to_string(), 
    subtitle: "Recommended activities include visits to zoos and gardens".to_string(),
    };
    let context = build_post(&p1);
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![root])
    .mount("/", StaticFiles::from("/Users/cherry/Documents/rust/blog/static"))
    .attach(Template::fairing())
    .launch();
}
