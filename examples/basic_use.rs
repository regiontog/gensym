macro_rules! assert_type_impl_clone {
    ($a:ty) => {
        gensym::gensym!{ _assert_type_impl_clone!{ $a } }
    };
}

macro_rules! _assert_type_impl_clone {
    ($gensym:ident, $a:ty) => {
        fn $gensym() where $a: Clone {
            unimplemented!()
        }
    };
}

mod test {
    struct MyStruct;

    assert_type_impl_clone!{ u64 }
    assert_type_impl_clone!{ MyStruct }
}

fn main() { }
