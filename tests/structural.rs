extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern {
                pub type Foo;

                #[wasm_bindgen(method, structural)]
                fn bar(this: &Foo);
                #[wasm_bindgen(method, getter, structural)]
                fn baz(this: &Foo) -> u32;
                #[wasm_bindgen(method, setter, structural)]
                fn set_baz(this: &Foo, val: u32);
            }

            #[wasm_bindgen]
            pub fn run(a: &Foo) {
                a.bar();
                assert_eq!(a.baz(), 1);
                a.set_baz(2);
                assert_eq!(a.baz(), 2);
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { run } from "./out";

            export function test() {
                let called = false;
                run({
                    bar() { called = true; },
                    baz: 1,
                });
                assert.strictEqual(called, true);
            }
        "#)
        .test();
}


