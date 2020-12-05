use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::CodeParser::new();
    let o = p.parse(&mut ctx, &mut out, "l:DW 0 PUSH lea dx , [5]");
    if let Err(e) = o {
        println!("{:?}", e);
    }
    println!("{:?}", out);
}
