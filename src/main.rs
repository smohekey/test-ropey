use ropey::Rope;

fn log_chunk(chunk: Option<&str>) {
    println!("chunk: {}", chunk.map_or("None", |c| c));
}

fn main() {
    let rope = Rope::from_str("text");
    let mut chunks = rope.chunks();

    let chunk = chunks.next();
    println!("next()");
    log_chunk(chunk);

    let chunk = chunks.next();
    println!("next()");
    log_chunk(chunk);

    let chunk = chunks.prev();
    println!("prev()");
    log_chunk(chunk);

    let chunk = chunks.next();
    println!("next()");
    log_chunk(chunk);
}
