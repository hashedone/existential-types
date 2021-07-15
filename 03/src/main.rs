#![feature(min_type_alias_impl_trait)]

type IdGen = impl Iterator<Item = i32>;

fn id_gen() -> IdGen {
    0..
}

struct Service {
    _ids: IdGen,
}

fn main() {
    let _service = Service { _ids: id_gen() };
}
