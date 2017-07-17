use i8x16;

#[allow(dead_code)]
extern "platform-intrinsic" {
    pub fn x86_mm_cmpestra(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpestrc(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpestri(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpestrm(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i8x16;
    pub fn x86_mm_cmpestro(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpestrs(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpestrz(x: i8x16, y: i32, z: i8x16, w: i32, a: i32) -> i32;
    pub fn x86_mm_cmpistra(x: i8x16, y: i8x16, z: i32) -> i32;
    pub fn x86_mm_cmpistrc(x: i8x16, y: i8x16, z: i32) -> i32;
    pub fn x86_mm_cmpistri(x: i8x16, y: i8x16, z: i32) -> i32;
    pub fn x86_mm_cmpistrm(x: i8x16, y: i8x16, z: i32) -> i8x16;
    pub fn x86_mm_cmpistro(x: i8x16, y: i8x16, z: i32) -> i32;
    pub fn x86_mm_cmpistrs(x: i8x16, y: i8x16, z: i32) -> i32;
    pub fn x86_mm_cmpistrz(x: i8x16, y: i8x16, z: i32) -> i32;
}
