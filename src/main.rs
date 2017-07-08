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

fn parse_html() {
    let html = String::from("<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">
<TITLE src=\"hello\">302 Moved</TITLE></HEAD><BODY>
<H1>302 Moved</H1>
The document has moved 忠犬ハチ公
<A HREF=\"https://www.google.co.uk/?gfe_rd=cr&amp;ei=PpxeWa34NcGN8QeJzIHYAg\">here</A>.
</BODY></HTML>");

    let url_attributes = vec!["href", "src"];

    let length = html.len();
    let mut pos = 0;

    let mut next_char = move | | {
        // Increment to pos by the number of bytes in the codepoint
        let mut iter = html[pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        pos += next_pos;
        return (pos, cur_char);
    };

    loop {
        let (mut pos, mut cur_char) = next_char();
        let mut cur_char_lower = cur_char.to_lowercase().last().unwrap();

        for a in url_attributes.clone() {
            let mut score = a.len();
            for ch in a.chars() {
                if ch == cur_char_lower {
                    score -= 1
                } else {
                    break;
                }

                // TODO(): Why can't I immediately store the result in the existing variables
                let (npos, ncur_char) = next_char();
                pos = npos;
                cur_char = ncur_char;
                cur_char_lower = cur_char.to_lowercase().last().unwrap();
           }
           if score == 0 {
                println!("SCOREEEEE {}, {}", a, pos)

                // Eat up until a quote or alpha character?

                loop {
                    let (npos, ncur_char) = next_char();
                    pos = npos;
                    cur_char = ncur_char;
                }
           }

        }

        if pos >= length { break }
    }

}

/*fn parse_html() {
    let html = String::from("<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">
<TITLE>302 Moved</TITLE></HEAD><BODY>
<H1>302 Moved</H1>
The document has moved 忠犬ハチ公
<A HREF=\"https://www.google.co.uk/?gfe_rd=cr&amp;ei=PpxeWa34NcGN8QeJzIHYAg\">here</A>.
</BODY></HTML>");

    let length = html.char_indices().count();
    let mut html_chars = html.char_indices();

    print!("Length={}\n", length);
/*
    let mut value_from_iterator = move |is_alpha: bool| {
        loop {
            let k = html_chars.next();
            match k {
                None => return (0, ' ', false),
                _ => {}
            }
            let (i, c): (usize, char) = k.unwrap();
            if is_alpha && !c.is_alphabetic() { continue }
            return (i, c, true);
        }
    };

    'main: loop {

        let (i, c, ok) = value_from_iterator(true);
        if !ok {
            break;
        }

        if c == '<' {

            let (i, c, ok) = value_from_iterator(true);
            if !ok {
                break;
            }

            if c == 'a' || c == 'A' {
                loop {
                    let (i, c, ok) = value_from_iterator(false);
                    if !ok {
                        break 'main;
                    }
                    println!("{}={}", i, c);
               }
            }
        }

    } */

    /*
    let mut in_quotes = false;
    let mut current_quote_type = ' ';
    let mut eating = false;
    let mut last_char = ' ';
    let mut in_decl = false;

    for c in html.chars() {

        if c == '<' {
            in_decl = true
        } else if in_decl
                && (c == 'a' || c == 'A')
                && (last_char == ' ' || last_char == '<') {
            eating = true
        } else if eating && !in_quotes && c == '>' {
            eating = false
        } else if eating {
            if !in_quotes && (c == '"' || c == '\''){
                current_quote_type = c;
                in_quotes = true;
            } else if in_quotes && c == current_quote_type {
                in_quotes = false;
            } else if in_quotes {
                print!("char: {}\n", c);
            }
        }

        last_char = c;


    }*/
}*/

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
