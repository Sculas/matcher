use std::{fs::read_to_string, path::Path};

fn base_test(path: &Path) -> datatest_stable::Result<()> {
    let _input = read_to_string(path)?;
    Ok(())
}

macro_rules! test {
    ($($name:ident),+) => {
        $(
            fn $name(path: &Path) -> datatest_stable::Result<()> {
                base_test(path)
            }
        )+

        datatest_stable::harness!($(
            $name, concat!("testdata/mappings/", stringify!($name)), r"^(.*)\.mapping"
        ),+);
    };
}

test!(yt, yarn);
