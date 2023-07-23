#[cfg(test)]
mod test {
    use parser::{parse_nginx_conf, parse_nginx_rule, Rule};
    use pest::Parser;

    #[test]
    fn comment() {
        let result = parse_nginx_rule(Rule::COMMENT, "#123123  \nasd".into());
        assert!(&result.is_ok());
    }

    #[test]
    fn value_string() {
        let result = parse_nginx_rule(Rule::value_string, "asdawef1 #asd".into());
        assert!(&result.is_ok());
    }

    #[test]
    fn value_number() {
        let result = parse_nginx_rule(Rule::value_number, "1".into());
        assert!(&result.is_ok());
        let result = parse_nginx_rule(Rule::value_number, "-1".into());
        assert!(&result.is_ok());
        let result = parse_nginx_rule(Rule::value_number, "-asd1".into());
        assert!(&result.is_err());
    }

    #[test]
    fn assignment_root() {
        let result = parse_nginx_rule(Rule::assignment_root, "worker_processes   1;".into());
        assert!(&result.is_ok());
    }

    #[test]
    fn block_location() {
        let result = parse_nginx_rule(Rule::block_location, "location / / {}".into());
        assert!(&result.is_err());
        let result = parse_nginx_rule(Rule::block_location, "location ~/index.html/{}".into());
        assert!(&result.is_ok());
        let result = parse_nginx_rule(
            Rule::block_location,
            "location ~/index.html/user/(.*){}".into(),
        );
        assert!(&result.is_ok());
        let result = parse_nginx_rule(
            Rule::block_location,
            r#"location ~/index.html/user/(.*)   
            {
                index index.html;
                proxy_redirect /index.html;
                try_files $uri $uri/ /index.html;
            }
            "#
            .into(),
        );
        eprintln!("{:#?}", &result);
        assert!(&result.is_ok());
    }

    #[test]
    fn root() {
        let content =
            std::fs::read_to_string("tests/nginx/nginx.conf").unwrap();
        let result = parse_nginx_rule(Rule::root, content);
        eprintln!("{:#?}", &result);
        assert!(&result.is_ok());
    }
}
