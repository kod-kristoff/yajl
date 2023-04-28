/*
 * Copyright (c) 2007-2014, Lloyd Hilaiel <me@lloyd.io>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

// #include <stdio.h>
// #include <string.h>

use std::io;
use std::io::Read;

use yajl::yajl_tree;

// static unsigned char fileData[65536];

fn main() -> io::Result<()> {
    // size_t rd;
    // yajl_val node;
    let mut fileData = [0; 65536];
    let mut errbuf = [0; 1024];

    /* null plug buffers */
    // fileData[0] = errbuf[0c] = 0;
    /* read the entire config file */
    // rd = fread((void *) fileData, 1, sizeof(fileData) - 1, stdin);
    let rd = io::stdin().read(&mut fileData)?;

    println!("read {rd}");
    /* file read error handling */
    // if (rd == 0 && !feof(stdin)) {
    //     fprintf(stderr, "error encountered on file read\n");
    //     return 1;
    // } else if (rd >= sizeof(fileData) - 1) {
    //     fprintf(stderr, "config file too big\n");
    //     return 1;
    // }

    /* we have the whole config file in memory.  let's parse it ... */
    // node = yajl_tree_parse((const char *) fileData, errbuf, sizeof(errbuf));
    let node = yajl_tree::parse(&fileData, &mut errbuf);

    /* parse error handling */
    // if (node == NULL) {
    //     fprintf(stderr, "parse_error: ");
    //     if (strlen(errbuf)) fprintf(stderr, " %s", errbuf);
    //     else fprintf(stderr, "unknown error");
    //     fprintf(stderr, "\n");
    //     return 1;
    // }

    /* ... and extract a nested value from the config file */
    // {
    //     const char * path[] = { "Logging", "timeFormat", (const char *) 0 };
    //     yajl_val v = yajl_tree_get(node, path, yajl_t_string);
    //     if (v) printf("%s/%s: %s\n", path[0], path[1], YAJL_GET_STRING(v));
    //     else   printf("no such node: %s/%s\n", path[0], path[1]);
    // }

    // yajl_tree_free(node);

    Ok(())
}
