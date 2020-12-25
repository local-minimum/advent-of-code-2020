static DIVIDER: usize  = 20201227;
static SUBJECT: usize = 7;

fn find_loop_size(public_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != public_key {
        value = value * SUBJECT % DIVIDER;
        loop_size += 1;
    }
    loop_size 
}

fn enc(loop_size: usize, subj: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subj % DIVIDER;
    }    
    value
}

fn find_encryption_key(key1: usize, key2: usize) -> usize {
    let loop1 = find_loop_size(key1);
    let loop2 = find_loop_size(key2);
    println!("Loop size {} / {}", loop1, loop2);
    let enc1 = enc(loop1, key2);
    let enc2 = enc(loop2, key1);
    println!("Enc {} / {}", enc1, enc2);
    enc1
}

fn main() {
    find_encryption_key(5764801, 17807724);
    let code = find_encryption_key(18499292, 8790390);
    println!("Code {}", code);
}
