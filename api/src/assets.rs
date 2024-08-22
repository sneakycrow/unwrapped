use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
#[include = "*.html"]
pub(crate) struct Assets;
