use arma_rs::{arma, Extension};
use once_cell::sync::Lazy;

mod discord;

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to start the runtime.")
});

#[arma]
fn init() -> Extension {
    let ext = Extension::build().command("discord", discord::send);
    ext.finish()
}
