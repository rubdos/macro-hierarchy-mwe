#[macro_use]
extern crate macro_hierarchy;

mod m {}

foo!( m );

#[test]
fn test_call_macro() {
}
