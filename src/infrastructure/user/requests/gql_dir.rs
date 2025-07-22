use include_dir::{include_dir, Dir};

pub static GQL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/gql");
