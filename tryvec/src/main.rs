fn make_vec(n: usize) -> Vec<u8> {
    vec![0; n]
}

fn main() {
    let v = vec![0; 10000];
    let v_len = v.len();
    println!("v_len={}", v_len);

    let v2 = make_vec(20000);
    let v2_len = v2.len();
    println!("v2_len={}", v2_len);

    let v3 = vec![1; 10];
    println!("v3[11]={}", v3[11]);
}

