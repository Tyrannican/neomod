use std::path::PathBuf;

// TODO: Add some form of basic app states

#[derive(Default, Debug)]
pub struct Neomod {
    pub store: PathBuf,
    pub wow_loc: PathBuf,
    pub quit: bool,
}
