use parser::parse_nginx_conf;

fn main() {
    let content =
        std::fs::read_to_string("tests/nginx/nginx.conf").unwrap();
    let _result = parse_nginx_conf(content);
}
