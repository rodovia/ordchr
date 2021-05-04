mod net;
mod meta;

use cursive::event::Key;
use cursive::views::*;
use cursive::traits::{Nameable, Resizable};
use cursive::theme::*;

#[tokio::main]
async fn main() {
    let mut app = cursive::default();

    front_step(&mut app).await;

    app.run();
}

async fn front_step(app: &mut cursive::Cursive) {
    let hh = net::gethost().await;

    app.add_global_callback(Key::Esc, |curse| curse.quit());
    app.add_layer(
        Dialog::new()
            .title(format!("ordchr {}", meta::VERSION))
            .content(
                ListView::new()
                    .child("Nome de Usuário", EditView::new().with_name("username").fixed_width(20))
            ).button("OK", |curse| connecting_step(curse))
    );
}

fn connecting_step(app: &mut cursive::Cursive) {
    app.pop_layer();

    let mut username = String::from("");
    app.call_on_name("username", |text: &mut EditView| {
        username = text.get_content().to_string();
    });

    let diag = Dialog::around(TextView::new(format!("Conectando como {}...", username)));
    app.add_layer(diag);
    tokio::runtime::Builder::new_multi_thread()
        .max_blocking_threads(1)
        .build()
        .unwrap()
        .block_on(net::connect(username));
}