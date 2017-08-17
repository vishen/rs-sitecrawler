use std::collections::HashMap;
use parser::Parser;

pub fn parse_html(html: String) -> HashMap<String, u32> {

    // TODO(): Ignore `url_attributes` in `<script>` tags

    let mut found_links: HashMap<String, u32> = HashMap::new();
    let mut p = Parser::new(html);
    let url_attributes = vec!["href", "src"];

    loop {
        let mut cur_char = p.next_char().to_lowercase().last().unwrap();

        // NOTE: This only works because neither attribute starts with the same letter
        for a in url_attributes.clone() {
            let mut score = a.len();
            for ch in a.chars() {
                if ch == cur_char {
                    score -= 1
                } else {
                    break;
                }

                if score == 0 {
                    // Check to see that the next char is a valid one of the end of an attribute
                    p.consume_whitespaces();

                    // Invalid attribute
                    if p.next_char() != '=' {
                        break;
                    }

                    p.consume_whitespaces();

                    let link = match p.peek_char() {
                        '"' | '\'' => {
                            let nc = p.next_char();
                            let test = |c| c == nc;
                            p.consume_until(test)
                        }
                        _ => {
                            let test = |c| match c {
                                '>' | ' ' | '\t' | '\n' => true,
                                _ => false,
                            };
                            p.consume_until(test)
                        }
                    };

                    // NOTE: Can't use `link` after this as `found_links.entry` now owns it
                    *found_links.entry(link).or_insert(0) += 1;
                } else {
                    cur_char = p.next_char().to_lowercase().last().unwrap();
;
                }
            }

        }

        if p.finished() {
            break;
        }
    }

    found_links

}


pub fn normalise_links() {
/*
    let test_cases: [String; 11] = [
        String::from("hello"),
        String::from("hello_world"),
        String::from("oneoneone"),
        String::from("&quot;//www.someurl.com/video-settings.svg&quot;&gt;"),
        String::from("https://www.someurl.com/?gfe_rd=cr&amp;ei=PpJzIHYAg"),
        String::from("one world"),
        String::from("/this_is_valid?"),
        String::from("extra123123?=hello"),
        String::from("//www.someurl.com/favicon.ico"),
        String::from("#content"),
        String::from("javascript:"),
    ];

    if starts with http:// or https:// then fine
    if starts with // then add http:
    if starts with / then add <base_url>/<rest>
    if starts with # then add <base_url>#<rest>
    if starts with alphanumeric and ends in alphanumeric or \ then add <base_url>/<rest>
    else ignore

    #TODO How to deal with &quot;//www.someurl.com/video-settings.svg&quot;&gt; ??
*/

    // TODO(): Write out the above!

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_html_for_links() {

        let html = String::from("<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">
<TITLE src=\"hello\">302 Moved</TITLE></HEAD><BODY>
<H1 href=hello_world>302 Moved</H1>
The document has moved 忠犬ハチ公 href=oneoneone
src=&quot;//www.someurl.com/video-settings.svg&quot;&gt;
<A HREF=\"https://www.someurl.com/?gfe_rd=cr&amp;ei=PpJzIHYAg\">here</A>.
<a hrefff=None>
<a href='one world'>
<a href = /this_is_valid?>
<a href = extra123123?=hello extra>
<a href         = \"hello_world\">
<a href=#content>
<a href=javascript: void 0;>
<a href=\"//www.someurl.com/favicon.ico\">
</BODY></HTML>");
        let found_links = parse_html(html);

        let test_cases: [String; 11] = [
            String::from("hello"),
            String::from("hello_world"),
            String::from("oneoneone"),
            String::from("&quot;//www.someurl.com/video-settings.svg&quot;&gt;"),
            String::from("https://www.someurl.com/?gfe_rd=cr&amp;ei=PpJzIHYAg"),
            String::from("one world"),
            String::from("/this_is_valid?"),
            String::from("extra123123?=hello"),
            String::from("//www.someurl.com/favicon.ico"),
            String::from("#content"),
            String::from("javascript:"),
        ];

        for t in test_cases.iter() {
            assert!(
                found_links.contains_key(t),
                "Couldn't find key '{}' in '{:?}", t, found_links
            );
        }

        assert!(
            found_links.len() == test_cases.len(),
            "Mismatched len; found_links={} test_cases={} ({:?})", found_links.len(), test_cases.len(), found_links
        );
    }
}
