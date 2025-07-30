pub fn is_power_of_2(n:usize) -> bool {
    let mut m:f32 = n as f32;
    while m != 1 as f32 {
        if (m as usize)%2 == 1 {
            return false
        }
        m /= 2 as f32;
        if m != m.round() {
            return false
        }
    }
    true
}