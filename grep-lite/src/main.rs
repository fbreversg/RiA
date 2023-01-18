fn main() {
   //Parameters
   let context_lines = 1;
   let needle = "oo";
   let haystack = "Every face, every shop,
   bedroom window, public-house, and
   dark square is a picture
   feverishly turned--in search of what?
   It is the same with books.
   What do we seek
   through millions of pages?";

    // Initialization
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    // Pass 1
    for (idx, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(idx);

            let v = Vec::with_capacity(2*context_lines + 1);
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        return;
    }
    
    // Pass 2
    for (idx, line) in haystack.lines().enumerate() {
        for (jdx, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (idx >= lower_bound) && (idx<=upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (idx, line_as_string);
                ctx[jdx].push(local_ctx);
            }
        }
    }

    // Output
    for local_ctx in ctx.iter() {
        for &(idx, ref line) in local_ctx.iter() {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }

}
