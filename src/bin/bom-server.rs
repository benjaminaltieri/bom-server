use ::bom_server::{make_rocket, SharedPartsList};

/// Use bom-server library to create a parts list and manage
/// with rocket based server reactor
fn main() {
    let parts_list = SharedPartsList::new();
    make_rocket(parts_list).launch();
}

