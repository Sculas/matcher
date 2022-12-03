use super::*;
use crate::utils::{MethodDescriptor, NodeExt};
use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "enigma.pest"]
pub struct EnigmaParser;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const PRINT_AST: bool = false;
macro_rules! prt_ast {
    ($input:expr) => {
        #[cfg(debug_assertions)]
        if PRINT_AST {
            dbg!($input.children());
        }
    };
}

#[pest_consume::parser]
impl parser::EnigmaParser {
    pub fn mappings(input: Node) -> Result<Mappings> {
        prt_ast!(input);
        Ok(match_nodes!(input.into_children();
            [class(classes).., _] => Mappings(classes.collect()),
        ))
    }

    fn class(input: Node) -> Result<ClassMapping> {
        Ok(match_nodes!(input.into_children();
            [name(obf), name(deobf), field_or_method(foms)..] => ClassMapping {
                obf,
                deobf: Some(deobf),
                methods: foms.clone().filter_map(FOM::method).collect(),
                fields: foms.filter_map(FOM::field).collect(),
            },
            [name(obf), field_or_method(foms)..] => ClassMapping {
              obf,
              deobf: None,
              methods: foms.clone().filter_map(FOM::method).collect(),
              fields: foms.filter_map(FOM::field).collect(),
          },
        ))
    }

    fn field_or_method(input: Node) -> Result<FOM> {
        Ok(match_nodes!(input.into_children();
            [method(method)] => FOM::Method(method),
            [field(field)] => FOM::Field(field),
        ))
    }

    fn method(input: Node) -> Result<MethodMapping> {
        Ok(match_nodes!(input.into_children();
            [name(obf), method_descriptor(descriptor), arg(arg_mappings)..] => MethodMapping {
                obf,
                deobf: descriptor.name,
                args: descriptor.args,
                ret: descriptor.ty,
                arg_mappings: arg_mappings.collect(),
            },
        ))
    }

    fn field(input: Node) -> Result<FieldMapping> {
        Ok(match_nodes!(input.into_children();
            [name(obf), name(deobf), descriptor(ty)] => FieldMapping {
                obf,
                deobf,
                ty,
            },
        ))
    }

    fn method_descriptor(input: Node) -> Result<MethodDescriptor> {
        Ok(match_nodes!(input.into_children();
            [name(name), descriptor(args).., descriptor(ty)] => MethodDescriptor {
                name: Some(name),
                args: args.collect(),
                ty,
            },
            [descriptor(args).., descriptor(ty)] => MethodDescriptor {
              name: None,
              args: args.collect(),
              ty,
          },
        ))
    }

    fn arg(input: Node) -> Result<ArgMapping> {
        Ok(match_nodes!(input.into_children();
            [number(index), name(deobf)] => ArgMapping {
                index,
                deobf,
            },
        ))
    }

    // string rules
    fn name(input: Node) -> Result<String> {
        Ok(input.str())
    }
    fn descriptor(input: Node) -> Result<String> {
        Ok(input.str())
    }
    fn number(input: Node) -> Result<i64> {
        input.int().map_err(|e| input.error(e))
    }

    // unused rules
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    fn jtype(_input: Node) -> Result<()> {
        Ok(())
    }
}

impl parser::EnigmaParser {
    pub fn mappings_into(input: Node, mappings: &mut Mappings) -> Result<()> {
        prt_ast!(input);
        match_nodes!(input.into_children();
            [class(classes).., _] => mappings.0.append(&mut classes.collect::<Vec<_>>()),
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test {
        ($name:ident, $file:expr, $fn:expr) => {
            #[test]
            fn $name() {
                let input = include_str!(concat!("../testdata/", $file));
                let res = match EnigmaParser::parse(Rule::mappings, input) {
                    Ok(mut pairs) => pairs.next().unwrap(),
                    Err(e) => panic!("{}", e),
                };
                $fn(res);
            }
        };
    }

    test!(standard, "standard.mapping", |node: Node| {
        assert_eq!(node.as_rule(), Rule::mappings);
        let class = node.into_children().next().unwrap();
        assert_eq!(class.as_rule(), Rule::class);
    });
}
