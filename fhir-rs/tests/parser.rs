
#[cfg(test)]
mod parser {
    use fhir_rs::prelude::*;

    #[test]
    pub fn test_parser() -> Result<()> {
        let mut tokenizer = Tokenizer::new("Patient.name[0].given[2]");

        let list = tokenizer.tokenize()?;

        let parser = Parser::new(list);
        let plist = parser.parse()?;

        for pt in plist.component {
            println!("{:?}", pt);
        };

        Ok(())
    }
}
