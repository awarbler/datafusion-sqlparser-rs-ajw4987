// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.use sqlparser::ast::Statement;
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
