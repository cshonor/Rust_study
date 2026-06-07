struct E(u32);
impl Drop for E { fn drop(&mut self) { println!("E({})", self.0); } }
fn main() {
    let v = vec![E(1), E(2), E(3)];
    println!("end scope");
}
