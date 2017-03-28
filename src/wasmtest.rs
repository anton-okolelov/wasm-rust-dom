extern crate webplatform;

fn main() {
    println!("============\nHello, world\n===============");
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set("<h1>Привет, Хабр, я web assembly!</h1> <button>Нажми меня</button>");
    let button = document.element_query("button").unwrap();
    button.on("click", |_| webplatform::alert("Кнопка нажата!"));
}