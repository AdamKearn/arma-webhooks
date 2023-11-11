use arma_rs::Group;

pub fn send() -> &'static str {
    "this is the responce back from discord...yayaya!"
}

pub fn group() -> Group {
    Group::new().command("send", send)
}
