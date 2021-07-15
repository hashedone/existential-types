fn id_gen() -> impl Iterator<Item = i32> {
    0..
}

struct Service<I> {
    _ids: I,
}

impl<I: Iterator<Item = i32>> Service<I> {
    // ...
}

fn main() {
    let _service = Service { _ids: id_gen() };
}
