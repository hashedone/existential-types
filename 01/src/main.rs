fn id_gen() -> impl Iterator<Item = i32> {
    0..
}

struct Service {
    _ids: Box<dyn Iterator<Item = i32>>,
}

fn main() {
    let _service = Service {
        _ids: Box::new(id_gen()),
    };
}
