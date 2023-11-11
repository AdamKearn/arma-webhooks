use arma_rs::{arma, Extension};

mod discord;

#[arma]
fn init() -> Extension {
    let ext = Extension::build().group("discord", discord::group());
    ext.finish()
}
