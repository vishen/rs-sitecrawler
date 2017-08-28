use std::collections::HashMap;
use parser::Parser;

pub fn parse_html(html: String) -> HashMap<String, u32> {

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

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Link {
    original: String,

    scheme: String,
    domain: String,

    path: Option<String>,
    query: Option<String>,
    hash: Option<String>,

    is_from_base_url: bool,
}

impl Link {
    pub fn url(&self) -> String {
        // TODO(): There HAS TO be a better way to do this!

        let mut _url = format!("{}//{}", self.scheme, self.domain);

        match self.path.clone() {
            Some(s) => {
                _url.push_str("/");
                _url.push_str(&s);
            }
            _ => (),
        }

        match self.query.clone() {
            Some(s) => {
                _url.push_str("?");
                _url.push_str(&s);
            }
            _ => (),
        }

        match self.hash.clone() {
            Some(s) => {
                _url.push_str("#");
                _url.push_str(&s);
            }
            _ => (),
        }

        _url
    }
}

pub fn normalise_links(base_url: &str, links: &HashMap<String, u32>) -> HashMap<Link, u32> {

    // TODO(): Re-write this is a parser!!!

    let base_url_split: Vec<&str> = base_url.splitn(2, "//").collect();
    let mut normalised_links: HashMap<Link, u32> = HashMap::new();

    // TODO(): @Hack This just seems wrong, but I can't figure out how this
    // should be done? Force the lifetime of a String created from "format!()"
    // to live longer
    let mut format_string_holder: String;

    for (link, count) in links {

        // Ignore known HTML javascript short-cuts
        if link == "javascript:" {
            continue;
        }

        let hash_split: Vec<&str> = link.splitn(2, "#").collect();
        let mut rest_of_url: &str = hash_split[0];
        let hash = match hash_split.len() {
            2 => Some(hash_split[1].to_string()),
            _ => None,
        };

        let query_split: Vec<&str> = rest_of_url.splitn(2, "?").collect();
        rest_of_url = query_split[0];
        let query = match query_split.len() {
            2 => Some(query_split[1].to_string()),
            _ => None,
        };


        let mut scheme_split: Vec<&str> = rest_of_url.splitn(2, "//").collect();
        // No scheme found
        if scheme_split.len() == 1 {
            // Add the base url
            // TODO(): @Cleanup - there must be a better way of doing this...?
            if scheme_split[0] == "" || scheme_split[0].chars().nth(0).unwrap() == '/' {
                format_string_holder = format!("{}{}", base_url, scheme_split[0]);
                rest_of_url = &format_string_holder;
            } else {
                format_string_holder = format!("{}/{}", base_url, scheme_split[0]);
                rest_of_url = &format_string_holder;
            }
            scheme_split = rest_of_url.splitn(2, "//").collect();
        } else if scheme_split[0] == "" || scheme_split[0] == ":" {
            format_string_holder = format!("http://{}", scheme_split[1]);
            rest_of_url = &format_string_holder;
            scheme_split = rest_of_url.splitn(2, "//").collect();
        }

        let scheme = scheme_split[0];
        rest_of_url = scheme_split[1];

        let path_split: Vec<&str> = rest_of_url.splitn(2, "/").collect();
        let path = match path_split.len() {
            2 => Some(path_split[1].to_string()),
            _ => None,
        };
        let domain = path_split[0];
        let is_from_base_url = domain.ends_with(base_url_split[1]);

        // println!("link={} -> scheme={}, domain={}, path={:?}, query={:?}, hash={:?}, is_from_base_url={}", link, scheme, domain, path, query, hash, is_from_base_url);

        let _link = Link {
            original: link.clone(),
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            path: path,
            query: query,
            hash: hash,
            is_from_base_url: is_from_base_url,
        };

        // println!("original={} ##### link={}", link, _link.url());

        normalised_links.insert(_link, *count);
    }


    normalised_links
}

#[test]
fn parse_html_for_links() {

    let html = String::from(
        "<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">
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
</BODY></HTML>",
    );
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
            "Couldn't find key '{}' in '{:?}",
            t,
            found_links
        );
    }

    assert!(
        found_links.len() == test_cases.len(),
        "Mismatched len; found_links={} test_cases={} ({:?})",
        found_links.len(),
        test_cases.len(),
        found_links
    );
}

#[test]
fn normalise_parsed_links() {

    struct TestCase {
        link: Link,
    }

    let test_cases: [TestCase; 9] = [
        TestCase {
            link: Link {
                original: "https://example.com/one?hello=world#id=1".to_string(),
                scheme: "https:".to_string(),
                domain: "example.com".to_string(),
                path: Some("one".to_string()),
                query: Some("hello=world".to_string()),
                hash: Some("id=1".to_string()),
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "http://d.example.com".to_string(),
                scheme: "http:".to_string(),
                domain: "d.example.com".to_string(),
                path: None,
                query: None,
                hash: None,
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "//example.com?hello=world".to_string(),
                scheme: "http:".to_string(),
                domain: "example.com".to_string(),
                path: None,
                query: Some("hello=world".to_string()),
                hash: None,
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "/path/somewhere".to_string(),
                scheme: "http:".to_string(),
                domain: "example.com".to_string(),
                path: Some("path/somewhere".to_string()),
                query: None,
                hash: None,
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "path/somewhere".to_string(),
                scheme: "http:".to_string(),
                domain: "example.com".to_string(),
                path: Some("path/somewhere".to_string()),
                query: None,
                hash: None,
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "#id123".to_string(),
                scheme: "http:".to_string(),
                domain: "example.com".to_string(),
                path: None,
                query: None,
                hash: Some("id123".to_string()),
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "ftp://bad.example.com".to_string(),
                scheme: "ftp:".to_string(),
                domain: "bad.example.com".to_string(),
                path: None,
                query: None,
                hash: None,
                is_from_base_url: true,
            },
        },

        TestCase {
            link: Link {
                original: "https://not.the.same.domain".to_string(),
                scheme: "https:".to_string(),
                domain: "not.the.same.domain".to_string(),
                path: None,
                query: None,
                hash: None,
                is_from_base_url: false,
            },
        },

        TestCase {
            link: Link {
                original: "://example.com#hello".to_string(),
                scheme: "http:".to_string(),
                domain: "example.com".to_string(),
                path: None,
                query: None,
                hash: Some("hello".to_string()),
                is_from_base_url: true,
            },
        },
    ];

    for test_case in test_cases.iter() {
        let _link = &test_case.link;

        let mut links: HashMap<String, u32> = HashMap::new();

        links.insert(_link.original.clone(), 1);

        let normalised_links = normalise_links("http://example.com", &links);

        assert!(
            normalised_links.len() == 1,
            "Normalised links didn't return a result"
        );

        let nl = normalised_links.keys().nth(0).unwrap();

        assert!(
            nl.original == _link.original,
            "{} != {}",
            nl.original,
            _link.original
        );
        assert!(
            nl.scheme == _link.scheme,
            "{} != {}",
            nl.scheme,
            _link.scheme
        );
        assert!(
            nl.domain == _link.domain,
            "{} != {}",
            nl.domain,
            _link.domain
        );
        assert!(nl.path == _link.path, "{:?} != {:?}", nl.path, _link.path);
        assert!(
            nl.query == _link.query,
            "{:?} != {:?}",
            nl.query,
            _link.query
        );
        assert!(nl.hash == _link.hash, "{:?} != {:?}", nl.hash, _link.hash);
        assert!(
            nl.is_from_base_url == _link.is_from_base_url,
            "{} != {}",
            nl.is_from_base_url,
            _link.is_from_base_url
        );

    }
}
