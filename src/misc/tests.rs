use super::sep_vec::SepVec;

#[test]
fn test_sep_vec() {
    let wow = SepVec::new("starting".to_string().chars(), 'A');
    println!("{:?}", wow.to_vec())
}