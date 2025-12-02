use sqlparser:: dialect::GenericDialect;
user sqlparser::parser::Parser;

pub fn to_cypher(sql: &str)-> String {
    let dialect = GenericDialect{};
    let ast = match Parser::parse_sql(&dialect, sql) {
        Ok(parsed) => parsed,
        Err(err) => {
            return format!(" Parse Error: {}", err);
        }    
    };
    format!("cypher for input\n{}", sql)
}