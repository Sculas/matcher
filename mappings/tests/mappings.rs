use std::{fs::read_to_string, path::Path};

use mappings::parser::{EnigmaParser, Rule};
use pest_consume::Parser;

macro_rules! test {
    ($($name:ident),+) => {
        $(
            fn $name(path: &Path) -> datatest_stable::Result<()> {
                match EnigmaParser::parse(Rule::mappings, &read_to_string(path)?) {
                    Err(e) => panic!("{}", e),
                    Ok(nodes) => {
                        let root = nodes.single()?;
                        let has_fields = root.as_str().contains("FIELD");
                        let has_methods = root.as_str().contains("METHOD");
                        let mappings = EnigmaParser::mappings(root)?;
                        assert!(mappings.classes().len() > 0);
                        for class in mappings.classes() {
                            assert!(class.obf.len() > 0);
                            if has_fields {
                                assert!(class.fields.len() > 0);
                            }
                            if has_methods {
                                assert!(class.methods.len() > 0);
                            }
                        }
                    }
                };
                Ok(())
            }
        )+

        datatest_stable::harness!($(
            $name, concat!("testdata/mappings/", stringify!($name)), r"^(.*)\.mapping"
        ),+);
    };
}

test!(yt, yarn);
