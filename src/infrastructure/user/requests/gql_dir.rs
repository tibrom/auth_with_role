use include_dir::{Dir, include_dir};

pub static GQL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/gql");