pub mod models {
    pub mod settings {
        include!(concat!(env!("OUT_DIR"), "/discord.settings.rs"));
    }
}
