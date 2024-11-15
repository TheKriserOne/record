use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, str::FromStr, sync::Mutex};

use auth::{AuthStore, Platform};
use backend::Messenger;
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;
use slint::ComponentHandle;
use crate::backend::utils::http_request;

mod auth;
mod backend;

slint::include_modules!();

fn main() {
    // Token Store
    let auth_store = Rc::new(RefCell::new(AuthStore::new("public/LoginInfo".into())));

    #[cfg(not(debug_assertions))]
    {
        #[cfg(unix)]
        {
            let stdout = File::create("/tmp/record.out").unwrap();
            let stderr = File::create("/tmp/record.err").unwrap();

            let daemonize = Daemonize::new()
                .pid_file("/tmp/record.pid")
                .stdout(stdout)
                .stderr(stderr);

            match daemonize.start() {
                Ok(_) => println!("Daemon started"),
                Err(e) => eprintln!("Error, {}", e),
            }
        }
    }

    let ui = MainWindow::new().unwrap();

    if !(*auth_store).borrow().is_empty() {
        let mut auth_store = (*auth_store).borrow_mut();

        let mut auths_to_remove = vec![];
        smol::block_on(async {
            for (i, auth) in auth_store.iter_mut().enumerate() {
                let messenger = auth.get_messanger();
                if let Err(_) = messenger.get_contacts().await {
                    auths_to_remove.push(i);
                } else {
                    ui.set_page(Page::Main)
                };
            }
        });

        auths_to_remove.sort_by(|a, b| b.cmp(a));
        auths_to_remove.iter().for_each(|i| auth_store.remove(*i));
    }

    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit({
        let ui = ui.clone_strong();
        let auth_store = auth_store.clone();
        move |string_auth| {
            let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
            let token = string_auth.token.to_string();
            (*auth_store)
                .borrow_mut()
                .add(Platform::from(platform), token);
            // TODO: Check if the token is valid before exiting form
            ui.set_page(Page::Main);
        }
    });

    ui.run().unwrap();
}
