use pest_derive::*;

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MDLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::error::Error as PError;
    use pest::iterators::Pairs;
    use pest::Parser;
    //assert_eq!("OUTPUT HERE", as_str(MDLParser::parse(Rule::<RULE>, "PARSE STRING")));

    fn as_str(parsed: Result<Pairs<'_, Rule>, PError<Rule>>) -> &str {
        dbg!(parsed).ok().map_or("", |s| s.as_str())
    }

    #[test]
    fn parse_float() {
        assert_eq!("", as_str(MDLParser::parse(Rule::float, "-NUMBER HERE 1.0")));
        assert_eq!("", as_str(MDLParser::parse(Rule::float, ".")));

        assert_eq!("1", as_str(MDLParser::parse(Rule::float, "1")));
        assert_eq!("-1", as_str(MDLParser::parse(Rule::float, "-1")));

        assert_eq!("1.", as_str(MDLParser::parse(Rule::float, "1.")));
        assert_eq!("-1.", as_str(MDLParser::parse(Rule::float, "-1.")));

        assert_eq!("1.2", as_str(MDLParser::parse(Rule::float, "1.2")));

        assert_eq!(".2", as_str(MDLParser::parse(Rule::float, ".2")));
        assert_eq!("-.2", as_str(MDLParser::parse(Rule::float, "-.2")));

        assert_eq!("1.2", as_str(MDLParser::parse(Rule::float, "1.2.3")));
        assert_eq!(".2", as_str(MDLParser::parse(Rule::float, ".2.3")));
    }

    #[test]
    fn parse_ident() {
        assert_eq!("PARSE", as_str(MDLParser::parse(Rule::ident, "PARSE")));
        assert_eq!("PARSE_STRING", as_str(MDLParser::parse(Rule::ident, "PARSE_STRING")));
        assert_eq!("", as_str(MDLParser::parse(Rule::ident, " parse")));
        assert_eq!("", as_str(MDLParser::parse(Rule::ident, "3PARSE")));
        assert_eq!("parse_st", as_str(MDLParser::parse(Rule::ident, "parse_st~uff")));
    }

    #[test]
    fn parse_translate() {
        assert_eq!("move 1 2. -.9", as_str(MDLParser::parse(Rule::translate, "move 1 2. -.9")));
        assert_eq!(
            "move 1 2 3 foobarbaz",
            as_str(MDLParser::parse(Rule::translate, "move 1 2 3 foobarbaz"))
        );
    }

    #[test]
    fn parse_rotate() {
        assert_eq!("rotate x 20", as_str(MDLParser::parse(Rule::rotate, "rotate x 20")));
        assert_eq!("rotate y -2 foo", as_str(MDLParser::parse(Rule::rotate, "rotate y -2 foo")));
        assert_eq!("", as_str(MDLParser::parse(Rule::rotate, "rotate xy 5")));
    }

    #[test]
    fn parse_box() {
        assert_eq!("box 1 2 3 4 5 6", as_str(MDLParser::parse(Rule::cuboid, "box 1 2 3 4 5 6")));
        assert_eq!(
            "box foo 1 2 3 4 5 6",
            as_str(MDLParser::parse(Rule::cuboid, "box foo 1 2 3 4 5 6"))
        );
        assert_eq!(
            "box 1 2 3 4 5 6 foo",
            as_str(MDLParser::parse(Rule::cuboid, "box 1 2 3 4 5 6 foo"))
        );
        assert_eq!(
            "box foo 1 2 3 4 5 6 bar",
            as_str(MDLParser::parse(Rule::cuboid, "box foo 1 2 3 4 5 6 bar"))
        );
        assert_eq!("", as_str(MDLParser::parse(Rule::cuboid, "box foo bar 1 2 3 4 5 6")));
    }

    #[test]
    fn parse_constants() {
        assert_eq!(
            "constants foo 1 2 3 4 5 6 7 8 9",
            as_str(MDLParser::parse(Rule::constants, "constants foo 1 2 3 4 5 6 7 8 9"))
        );
        assert_eq!(
            "constants foo 1 2 3 4 5 6 7 8 9 10 11 12",
            as_str(MDLParser::parse(Rule::constants, "constants foo 1 2 3 4 5 6 7 8 9 10 11 12"))
        );
        assert_eq!(
            "constants foo 1 2 3 4 5 6 7 8 9 ",
            as_str(MDLParser::parse(Rule::constants, "constants foo 1 2 3 4 5 6 7 8 9 10 11"))
        );
    }

}
