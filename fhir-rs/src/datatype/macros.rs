
#[macro_export]
macro_rules! impl_element {
    ($typ: ident) => {
        impl Element for $typ {

            fn id(&self) -> Option<&String> {
                self.id.as_ref()
            }

            fn set_id<T: Into<String>>(mut self, id: T) -> Self {
                self.id = Some(id.into());
                self
            }

            fn extensions(&self) -> Option<&Vec<Extension>> {
                self.extension.as_ref()
            }

            fn set_extensions(mut self, ext: Vec<Extension>) -> Self {
                self.extension = Some(ext);
                self
            }

            fn add_extension(mut self, ext: Extension) -> Self {
                match self.extension {
                    Some(ref mut exts) => exts.push(ext),
                    None => self.extension = Some(vec![ext]),
                }
                self
            }
        }

        impl Base for $typ {
            fn type_name(&self) -> &str {
                stringify!($typ)
            }
        }
    };
}
