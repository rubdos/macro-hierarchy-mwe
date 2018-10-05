mod foobar {
    macro_rules! bar {
        ($i:ident) => {struct Bar;};
    }

    #[macro_export]
    macro_rules! foo {
        ($j:ident) => {
            bar!($j);
        };
    }
}
