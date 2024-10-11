fn main() {
    let file = std::fs::read_to_string("textfun_in").unwrap();

    let mut out = String::new();
    let mut template = String::new();

    let mut template_state = 0;
    for line in file.lines() {
        match line {
            "${" => {
                if template_state != 0 {
                    panic!("invalid template state");
                }
                template_state = 1;
            }
            "$}" => {
                if template_state != 1 {
                    panic!("you forgot to open the template!");
                }
                template_state = 2;
            }
            "Type = {" => {
                template_state = 3;
            }
            "}" => {
                template_state = 4;
            }
            _ => {
                match template_state {
                    1 => {
                        template.push_str(line);
                        template.push('\n');
                    }
                    3 => {
                        out.push_str(&template.replace("$Type", line));
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{}", out);
    std::fs::write("textfun_out", out).unwrap();
}
