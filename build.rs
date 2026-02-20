use vergen_git2::{Git2Builder, Emitter};

fn main() {
    Emitter::default()
        .add_instructions(&Git2Builder::default().sha(true).build().unwrap())
        .unwrap()
        .emit()
        .unwrap();
}
