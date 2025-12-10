use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[test]
fn parse_simple_cypher() {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, "MATCH (n) RETURN n").unwrap();

    match &ast[0] {
        Statement::Cypher(c) => {
            assert_eq!(c.pattern, "(n)");
            assert_eq!(c.projections, vec!["n"]);
        }
        _ => panic!("not cypher"),
    }
}
