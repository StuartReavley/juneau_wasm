use juneau_core::{assert_eq_object, hash_map};
use super::Template;


#[test]
fn template_render() {
    assert_eq_object!(Template::render("Hello ${name}", hash_map!{"name".into() => "bob".into()}), "Hello bob");
}