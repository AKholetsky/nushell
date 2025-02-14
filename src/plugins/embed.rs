use nu::{
    serve_plugin, CallInfo, Plugin, Primitive, ReturnSuccess, ReturnValue, ShellError, Signature,
    SyntaxType, Tag, Tagged, TaggedDictBuilder, Value,
};

struct Embed {
    field: Option<String>,
    values: Vec<Tagged<Value>>,
}
impl Embed {
    fn new() -> Embed {
        Embed {
            field: None,
            values: Vec::new(),
        }
    }

    fn embed(&mut self, value: Tagged<Value>) -> Result<(), ShellError> {
        match value {
            Tagged { item, tag } => match &self.field {
                Some(_) => {
                    self.values.push(Tagged {
                        item: item,
                        tag: tag,
                    });
                    Ok(())
                }
                None => Err(ShellError::string(
                    "embed needs a field when embedding a value",
                )),
            },
        }
    }
}

impl Plugin for Embed {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("embed")
            .desc("Embeds a new field to the table.")
            .required("Field", SyntaxType::String)
            .rest(SyntaxType::String)
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        if let Some(args) = call_info.args.positional {
            match &args[0] {
                Tagged {
                    item: Value::Primitive(Primitive::String(s)),
                    ..
                } => {
                    self.field = Some(s.clone());
                    self.values = Vec::new();
                }
                _ => {
                    return Err(ShellError::string(format!(
                        "Unrecognized type in params: {:?}",
                        args[0]
                    )))
                }
            }
        }

        Ok(vec![])
    }

    fn filter(&mut self, input: Tagged<Value>) -> Result<Vec<ReturnValue>, ShellError> {
        self.embed(input)?;
        Ok(vec![])
    }

    fn end_filter(&mut self) -> Result<Vec<ReturnValue>, ShellError> {
        let mut root = TaggedDictBuilder::new(Tag::unknown());
        root.insert_tagged(
            self.field.as_ref().unwrap(),
            Tagged {
                item: Value::List(self.values.clone()),
                tag: Tag::unknown(),
            },
        );
        Ok(vec![ReturnSuccess::value(root.into_tagged_value())])
    }
}

fn main() {
    serve_plugin(&mut Embed::new());
}
