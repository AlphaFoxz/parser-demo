use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "pest_home/nginx.conf.pest"]
struct NginxParser;

pub fn parse_nginx_rule(rule: Rule, content: String) -> Result<(), pest::error::Error<Rule>> {
    let _result = NginxParser::parse(rule, content.as_str())?;
    Ok(())
}
pub fn parse_nginx_conf(content: String) -> Result<(), pest::error::Error<Rule>> {
    let result = NginxParser::parse(Rule::root, content.as_str())?;
    for pair in result.into_iter() {
        match pair.as_rule() {
            Rule::root => {
                eprintln!("===============检测到root规则===============");
                eprintln!("{:#?}", pair);
            }
            Rule::assignment_root => {
                eprintln!("===============检测到assignment_root规则===============");
                eprintln!("{:#?}", pair); 
            }
            _ => {
                eprintln!("{:#?}", pair);
            }
        }
    }
    Ok(())
}
