const HSH_BASE: u64 = 57;
const HSH_MOD: u64 = 1e9 as u64 + 7;
use std::{fmt::Display, ops::Range, path::Path};
pub struct Code {
    pub code: String,
    pub hsh: u32,
}
// pub struct Object {
//     name: String,
//     code_name: String,
// }
impl Code {
    pub fn update(&mut self, text: String) {
        self.code.push_str(text.as_ref());
    }
    pub fn create_wire(&mut self, prefix: &String, name: &String, postfix: &String) -> String {
        self.hsh = ((self.hsh + 1) as u64 * HSH_BASE % HSH_MOD) as u32;
        let var_name = format!("{}__{:x}", name, self.hsh);
        self.update(format!(
            "
wire {prefix}{var_name}{postfix};"
        ));
        var_name
    }
}
// impl Object {
//     fn new_raw(name: String) -> Object {
//         Object {
//             code_name: name.clone(),
//             name,
//         }
//     }
//     fn new(code: &mut Code, bits: Range<u8>, name: String) -> Object {
//         let code_name = code.create_wire(
//             &format!("[{} : {}]", bits.start, bits.end),
//             &name,
//             &"".to_string(),
//         );
//         Object { name, code_name }
//     }
//     fn new_array(code: &mut Code, bits: Range<u8>, name: String, range: Range<u8>) -> Object {
//         let code_name = code.create_wire(
//             &format!("[{} : {}]", bits.start, bits.end),
//             &name,
//             &format!("[{} : {}]", bits.start, bits.end),
//         );
//         Object { name, code_name }
//     }
//     fn substitute_range(&self, range: Range<u8>) -> String {
//         format!("{}[{} : {}]", self.code_name, range.start, range.end)
//     }
//     fn substitute(&self, index: u8) -> String {
//         format!("{}[{}]", self.code_name, index)
//     }
// }
// impl AsRef<String> for Object {
//     fn as_ref(&self) -> &String {
//         &self.code_name
//     }
// }
// impl Display for Object {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.as_ref().fmt(f)
//     }
// }
pub fn impl_buf(code: &mut Code, i: String, o: String) {
    code.update(format!(
        "
buf({} , {});",
        o, i
    ));
}
pub fn impl_bufif1(code: &mut Code, c: String, i: String, o: String) {
    code.update(format!(
        "
bufif1({} , {} , {});",
        o, i, c
    ));
}
