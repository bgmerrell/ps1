// Exercise 2.3
// I were better to be eaten to death with a rust than to be scoured to nothing with perpetual motion.
use std::os;
use std::io::File;

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
        ret.push(a[i] ^ b[i]);
    }
    ret
}

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } else {
        let fname1 = &args[1];
        let fname2 = &args[2];
        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());
        let share_file1 = File::open(&path1);
        let share_file2 = File::open(&path2);

        match (share_file1, share_file2) {
            (Some(mut share1), Some(mut share2)) => {
                let share1bytes: ~[u8] = share1.read_to_end();
                let share2bytes: ~[u8] = share2.read_to_end();
                print!("{:s}", std::str::from_utf8_owned(
                        xor(share1bytes, share2bytes)));
            } ,
            (_, _) => fail!("Error opening input files!")
        }
    }
}

