extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

use std::io::{self, Write};

use futures::Future;
use futures::future;
use futures::stream::Stream;

use hyper::{Client, Error};
use hyper_tls::HttpsConnector;

/*
    - unwrap():
        - https://github.com/rust-lang/rust/blob/09e2ad13d0aa01143bcb20dece3ff6c5a7e34ea3/src/libcore/option.rs#L329-L359
        - Semantically, it means "While this may or may not have a value, I'm saying it does. If it doesn't, just crash, I don't want to handle the error."
*/

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {

    fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input: input,
        }
    }

    // Read the current character without consuming it.
    fn peek_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn next_char(&mut self) -> char {
        // Increment to pos by the number of bytes in the codepoint
        // TODO: How does this work?
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        let lower_char = cur_char.to_lowercase().last().unwrap();
        lower_char
    }

    fn consume_until<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
    // fn consume_until(&mut self, c: char) -> String {
        let mut result = String::new();
        while !self.finished() && !test(self.peek_char()) {
            result.push(self.next_char());
        }

        result
    }

    fn consume_whitespaces(&mut self) {
        let test = |c| {
            match c {
                ' ' | '\t' | '\n' => false,
                _ => true
            }
        };
        self.consume_until(test);
    }

    fn finished(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn parse_html() {
    let html = String::from("<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">
<TITLE src=\"hello\">302 Moved</TITLE></HEAD><BODY>
<H1>302 Moved</H1>
The document has moved 忠犬ハチ公
<A HREF=\"https://www.google.co.uk/?gfe_rd=cr&amp;ei=PpxeWa34NcGN8QeJzIHYAg\">here</A>.
<a hrefff=None>
<a href='one world'>
<a href = /this_is_valid?>
<a href = extra123123?=hello extra>
<a href         = \"hello_world\">
</BODY></HTML>");

    let mut p = Parser::new(html);

    let url_attributes = vec!["href", "src"];

    loop {
        let mut cur_char = p.next_char();

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
                    // Check to see that the next char is a valid one of the end of an attrbute
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
                        },
                        _ => {

                            let test = |c| {
                                match c {
                                    '>' | ' ' | '\t' | '\n' => true,
                                    _ => false
                                }
                            };

                            p.consume_until(test)
                        }
                    };

                    println!("link={}", link);
               } else {
                    cur_char = p.next_char();
               }
           }

        }

        if p.finished() { break }
    }

}

fn main() {

    parse_html();
    return;


    let url = "https://google.com".parse::<hyper::Uri>().unwrap();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let client = Client::configure()
                        .connector(HttpsConnector::new(4, &handle).unwrap())
                        .build(&handle);

    // client.get -> hyper::client::FutureResponse
    // .and_then -> Execute another future after this one has resolved successfully
    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        /*
        // .for_each() -> Runs this stream to completion, executing the provided closure for each element on the stream.
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)

        })
        */

        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            print!("BODY: {}\n", s);
            future::ok::<_, Error>(0)
        })

    });

    core.run(work).unwrap();
}
