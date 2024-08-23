fn main(){
    let num = 5;
    find_complement(num)
}
pub fn find_complement(num: i32) {
    let binary = format!("{:b}", num);
    let mut vec_binary: Vec<char> = binary.chars().collect::<Vec<_>>();
    let mut ans:String = "".to_string();
    println!("{:?}", vec_binary);
        for i in 0..vec_binary.len(){
            if vec_binary[i] == '0'{
                ans = ans + "1";       
        }else {
            ans = ans + "0";
        }
    }
        println!("{:?}", ans );
        let ans2 = binary_string_to_decimal(&ans).unwrap();
        println!("{}", ans2)
}
fn binary_string_to_decimal(binary_string: &str) -> Result<i32, String> {
    let mut decimal_value = 0;
    let base = 2;

    for (i, char) in binary_string.chars().rev().enumerate() {
        let digit = char.to_digit(base).unwrap();
        decimal_value += digit * base.pow(i as u32);
    }

    Ok(decimal_value as i32)
}
