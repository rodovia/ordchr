mod net;
mod meta;
mod util;

use cursive::event::Key;
use cursive::views::*;
use cursive::traits::{Nameable, Resizable};

use log::{info};
use tokio::runtime::Handle;
use tokio::runtime::Runtime;

#[tokio::main]
#[allow(unused_must_use)]
async fn main() {
    util::setup_logger();

    info!("Hello cursive");
    let mut app = cursive::default();

    front_step(&mut app).await;

    app.run();
}

async fn front_step(app: &mut cursive::Cursive) {
    let _hh = net::gethost().await;

    app.add_global_callback(Key::F12, |curse| curse.toggle_debug_console());
    app.add_global_callback(Key::Esc, |curse| curse.quit());
    app.add_layer(
        Dialog::new()
            .title(format!("ordchr {}", meta::VERSION))
            .content(
                ListView::new()
                    .child("Nome de Usuário", EditView::new().with_name("username").fixed_width(20))
                    .child("IP do destino", EditView::new().with_name("ipaddr").fixed_width(10))
            ).button("OK", |curse| connecting_step(curse))
    );
}

fn connecting_step(app: &mut cursive::Cursive) {
    let mut username = String::from("");
    let mut ipaddr: Option<std::net::IpAddr> = None;
    app.call_on_name("username", |text: &mut EditView| {
        username = text.get_content().to_string();
    });

    app.call_on_name("ip", |text: &mut EditView| {
        let content = text.get_content().to_string();
        let ipv4: std::net::Ipv4Addr = content.parse().unwrap();
        ipaddr = Some(std::net::IpAddr::V4(ipv4));
    });

    app.pop_layer();

    let diag = Dialog::around(TextView::new(format!("Conectando como {}...", username)));
    app.add_layer(diag);
    info!("Connecting as {}...", username);

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let handle = rt.handle();
        handle.block_on(async {
            let ip = &ipaddr;
            net::connect_from_str(username.as_str(), ip.unwrap()).await;
        });
    });
}