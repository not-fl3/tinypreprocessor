extern crate regex;

use regex::*;

fn preprocess_wrapper<F : FnMut(&Captures) -> String>(text : &str, resolver : F) -> String {
    let re = Regex::new("(?m)^\\s*\\#include\\s+[\"<]([^\">]+)*[\">]").unwrap();

    re.replace_all(text, resolver)
}
pub fn preprocess<F : Fn(&str) -> String>(text : &str, resolver : F) -> String {
    preprocess_wrapper(text, |capture| resolver(capture.at(1).unwrap()))
}


#[test()]
fn test_include() {
    let glsl = r#"
aa
aa
#include "some1.glsl"
#include "some2.glsl"
"#;
    let some1 = "bb";
    let some2 = "cc";

    assert_eq!(preprocess(&glsl, |name| match name {
        "some1.glsl" => some1.to_string(),
        "some2.glsl" => some2.to_string(),
        _ => panic!("unknown path")
    }), r#"
aa
aa
bb
cc
"#);
}
