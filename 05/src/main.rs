#![feature(min_type_alias_impl_trait)]
#![feature(step_trait)]

type GenId<T: std::iter::Step> = impl Iterator<Item = T>;

fn gen_id<T: std::iter::Step>(from: T, to: T) -> GenId<T> {
    from..=to
}

struct Service<Id: std::iter::Step> {
    _ids: GenId<Id>,
}

fn main() {
    let _service = Service {
        _ids: gen_id(0, 100),
    };
}
