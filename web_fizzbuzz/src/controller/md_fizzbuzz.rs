
// fizzbuzz module
pub fn calc_fizzbuzz(_infig: i32) -> String {

    let mut ret =_infig.to_string();
    let tag = _infig;

    if tag % 15 == 0 {
        ret = "fizzbuzz".to_string()
    } else if tag % 3 == 0 {
        ret = "fizz".to_string()
    } else if tag % 5 == 0 {
        ret = "buzz".to_string()
    } else {
        // そのまま出力
    }

    ret
}
