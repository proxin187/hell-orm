

pub trait Model {
    const CREATE: &'static str;
}

pub trait Schema {
    fn sqls() -> &'static [&'static str];
}

impl Schema for () {
    fn sqls() -> &'static [&'static str] {
        &[]
    }
}

impl<Head: Model, Tail: Schema> Schema for (Head, Tail) {
    fn sqls() -> &'static [&'static str] {
        [&[Head::CREATE], Tail::sqls()].concat().leak()
    }
}

#[macro_export]
macro_rules! models {
    [] => { () };
    [$head:ty $(, $tail:ty)* $(,)?] => {
        ($head, models![$($tail),*])
    };
}


