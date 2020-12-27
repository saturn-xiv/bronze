extern crate bronze;

use bronze::{gua64::Gua64, gua8::Gua8, xiang4::Xiang4, xing5::Xing5, yi2::Yi2};

#[test]
fn test_yi2() {
    let begin: u16 = Yi2::Yang.into();
    for it in 0..2 {
        let it = Yi2::new(begin + it).unwrap();
        println!("{:?} => {}", it, it);
    }
}

#[test]
fn test_xiang4() {
    let begin: u16 = Xiang4::TaiYang.into();
    for it in 0..4 {
        let it = Xiang4::new(begin + it).unwrap();
        println!("{:?} => {}", it, it);
    }
}

#[test]
fn test_gua8() {
    let begin: u16 = Gua8::Qian.into();
    for it in 0..8 {
        let it = Gua8::new(begin + it).unwrap();
        println!("{:?} => {}", it, it);
    }
}

#[test]
fn test_xing5() {
    for it in vec![Xing5::Jin, Xing5::Mu, Xing5::Shui, Xing5::Huo, Xing5::Tu] {
        println!("{:?} => {}", it, it);
    }
}

#[test]
fn test_gua64() {
    let begin: u16 = Gua64::QianWeiTian.into();
    for it in 0..64 {
        let it = Gua64::new(begin + it).unwrap();
        println!("{:?} => {}", it, it);
    }
}
