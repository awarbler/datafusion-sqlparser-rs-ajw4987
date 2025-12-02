/**
Anita Woodford ajw4987
Module : cypher 
Purpose: provide a function that will take sql text and parser it by using the datafusion sql pars and return a cypher placeholder as the output. 
**/

 
use sqlparser::dialect::GenericDialect; // GD understand ansi style sql 
// uses the sql parser from the datafusion sql parser crate
use sqlparser::parser::Parser;

/**
Function: to_cypher 
input: &str is raw sql text from go server rust transformer 
output: string in cypher text currently mocked 

Part 3 of assignment 
1. accept sql text
2. run datafusion parser on it
4. return cypher output mocked is allowed 
**/
pub fn to_cypher(sql: &str)-> String {
    let dialect = GenericDialect{}; // create an instance 
    // parse the incoming sql string into ast if fails return an error 
    let ast = match Parser::parse_sql(&dialect, sql) {
        // if successful parse and store the result in parsed
        Ok(parsed) => parsed,
        Err(err) => {
            return format!(" Parse Error: {}", err);
        }    
    };
    // if parsed into ast returning mock string 
    format!("cypher for input\n{}", sql)
}