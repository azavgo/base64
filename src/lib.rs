use std::collections::HashMap;

fn base64_table() -> HashMap<(u8, String), char> {
    let mut base64_table: HashMap<(u8, String), char> = HashMap::new();
    
    base64_table.insert((0, "000000".to_string()), 'A');
    base64_table.insert((1, "000001".to_string()), 'B');
    base64_table.insert((2, "000010".to_string()), 'C');
    base64_table.insert((3, "000011".to_string()), 'D');
    base64_table.insert((4, "000100".to_string()), 'E');
    base64_table.insert((5, "000101".to_string()), 'F');
    base64_table.insert((6, "000110".to_string()), 'G');
    base64_table.insert((7, "000111".to_string()), 'H');
    base64_table.insert((8, "001000".to_string()), 'I');
    base64_table.insert((9, "001001".to_string()), 'J');
    base64_table.insert((10, "001010".to_string()), 'K');
    base64_table.insert((11, "001011".to_string()), 'L');
    base64_table.insert((12, "001100".to_string()), 'M');
    base64_table.insert((13, "001101".to_string()), 'N');
    base64_table.insert((14, "001110".to_string()), 'O');
    base64_table.insert((15, "001111".to_string()), 'P');
    base64_table.insert((16, "010000".to_string()), 'Q');
    base64_table.insert((17, "010001".to_string()), 'R');
    base64_table.insert((18, "010010".to_string()), 'S');
    base64_table.insert((19, "010011".to_string()), 'T');
    base64_table.insert((20, "010100".to_string()), 'U');
    base64_table.insert((21, "010101".to_string()), 'V');
    base64_table.insert((22, "010110".to_string()), 'W');
    base64_table.insert((23, "010111".to_string()), 'X');
    base64_table.insert((24, "011000".to_string()), 'Y');
    base64_table.insert((25, "011001".to_string()), 'Z');
    base64_table.insert((26, "011010".to_string()), 'a');
    base64_table.insert((27, "011011".to_string()), 'b');
    base64_table.insert((28, "011100".to_string()), 'c');
    base64_table.insert((29, "011101".to_string()), 'd');
    base64_table.insert((30, "011110".to_string()), 'e');
    base64_table.insert((31, "011111".to_string()), 'f');
    base64_table.insert((32, "100000".to_string()), 'g');
    base64_table.insert((33, "100001".to_string()), 'h');
    base64_table.insert((34, "100010".to_string()), 'i');
    base64_table.insert((35, "100011".to_string()), 'j');
    base64_table.insert((36, "100100".to_string()), 'k');
    base64_table.insert((37, "100101".to_string()), 'l');
    base64_table.insert((38, "100110".to_string()), 'm');
    base64_table.insert((39, "100111".to_string()), 'n');
    base64_table.insert((40, "101000".to_string()), 'o');
    base64_table.insert((41, "101001".to_string()), 'p');
    base64_table.insert((42, "101010".to_string()), 'q');
    base64_table.insert((43, "101011".to_string()), 'r');
    base64_table.insert((44, "101100".to_string()), 's');
    base64_table.insert((45, "101101".to_string()), 't');
    base64_table.insert((46, "101110".to_string()), 'u');
    base64_table.insert((47, "101111".to_string()), 'v');
    base64_table.insert((48, "110000".to_string()), 'w');
    base64_table.insert((49, "110001".to_string()), 'x');
    base64_table.insert((50, "110010".to_string()), 'y');
    base64_table.insert((51, "110011".to_string()), 'z');
    base64_table.insert((52, "110100".to_string()), '0');
    base64_table.insert((53, "110101".to_string()), '1');
    base64_table.insert((54, "110110".to_string()), '2');
    base64_table.insert((55, "110111".to_string()), '3');
    base64_table.insert((56, "111000".to_string()), '4');
    base64_table.insert((57, "111001".to_string()), '5');
    base64_table.insert((58, "111010".to_string()), '6');
    base64_table.insert((59, "111011".to_string()), '7');
    base64_table.insert((60, "111100".to_string()), '8');
    base64_table.insert((61, "111101".to_string()), '9');
    base64_table.insert((62, "111110".to_string()), '+');
    base64_table.insert((63, "111111".to_string()), '/');

    base64_table
}

//1. Extract individual characters from input s: &str

fn string_characters(s: &str) -> Vec<char> {
    s.chars().collect()
}

//2. Represent an input as UTF-8 bytes in a binary form

fn string_bytes(c: Vec<char>) -> String {
    let v_s: Vec<String> = c.iter().map(|e| character_bytes(*e)).collect(); 
    v_s.concat()
}

fn character_bytes(c: char) -> String {
    let n = c.len_utf8(); 
    let mut v = vec![0; n];
    let _s = c.encode_utf8(&mut v[..]); 
    let v_s: Vec<String> = v.iter().map(|e| format!("{:08b}", e)).collect(); 
    v_s.concat()
}
//3. Group the binary values into 6-bit chunks

fn sixbit_chunks(s: String) -> Vec<String> {
    let c: Vec<char> = s.chars().collect();
        let mut data: Vec<u8> = c
            .iter()
            .map(|e| e.to_string().parse::<u8>().unwrap())
            .collect();

        let m = data.len() % 6;
        if m != 0 {
            for _i in 0..6 - m {
                data.push(0);
            }
        }

        let mut v: Vec<String> = Vec::new();
        let mut x: Vec<String> = Vec::new();

        for e in data.windows(6).step_by(6) {
            for _i in 0..6 {
                x.push(e[_i].to_string())
            }
            v.push(x.concat());
            x.drain(..);
        }

        v
}

//4. Convert each individual 6-bit chunk into a decimal number

fn sixbit_decimal(v: Vec<String>) -> Vec<u8> {
    v.iter()
        .map(|e| u8::from_str_radix(&e, 2).unwrap())
        .collect()
}

//5. Represent each individual decimal number as base64 character

fn decimal_base64(v: Vec<u8>) -> Vec<char> {
    
    let base64_table = base64_table(); 
    let mut v_c: Vec<char> = Vec::new();

    for e in v {
        for (key, value) in &base64_table {
            if key.0 == e {
                v_c.push(*value)
            }
        }
    }
    v_c
}

//6. Represent ASCII characters as a String

fn vec_char_str(v: Vec<char>) -> String {
    v.iter().collect()
}

//Note: if number of bits in the last 6-bit chunk is less than 6, add a padding "=",
//or two paddings "=="

pub fn base64_encode(s: &str) -> String {   
    let v_1 = string_characters(s);
    let s_2 = string_bytes(v_1);
    let v_3 = sixbit_chunks(s_2);
    let v_4 = sixbit_decimal(v_3);
    let v_5 = decimal_base64(v_4); 
    let s_6 = vec_char_str(v_5);
    s_6
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test01_base64_encode() {
        assert_eq!("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string(), base64_encode("Many hands make light work."));
    }

    #[test]
    fn test02_base64_encode() {
        assert_eq!("0KfQsNC5IC0g0Y3RgtC+INCy0LrRg9GB0L3QviE".to_string(), base64_encode("Чай - это вкусно!"));
    }

    #[test]
    fn test03_base64_encode() {
        assert_eq!("44GK6Iy244GM576O5ZGz44GX44GE77yB".to_string(), base64_encode("お茶が美味しい！"));
    }
    
    #[test]
    fn test01_vec_char_str() {
        assert_eq!("H".to_string(), vec_char_str(vec!['H']));
    }

    #[test]
    fn test02_vec_char_str() {
        assert_eq!("TWFu".to_string(), vec_char_str(vec!['T','W','F','u']));
    }

    #[test]
    fn test01_decimal_base64() {
        assert_eq!(vec!['H'], decimal_base64(vec![7]));
    }

    #[test]
    fn test02_decimal_base64() {
        assert_eq!(vec!['H', 'j'], decimal_base64(vec![7, 35]));
    }


    #[test]
    fn test01_sixbit_decimal() {
        assert_eq!(vec![42], sixbit_decimal(vec!["101010".to_string()]));
    }

    #[test]
    fn test02_sixbit_decimal() {
        assert_eq!(vec![42, 25, 28], sixbit_decimal(vec!["101010".to_string(), "011001".to_string(), "011100".to_string()]));
    }

    #[test]
    fn test01_sixbit_chunks() {
        assert_eq!(vec!["101010".to_string(), "011001".to_string(), "011100".to_string()], sixbit_chunks("101010011001011100".to_string()));
    }

    #[test]
    fn test02_sixbit_chunks() {
        assert_eq!(vec!["101010".to_string()], sixbit_chunks("101010".to_string()));
    }

    #[test]
    fn test03_sixbit_chunks() {
        assert_eq!(vec!["101010".to_string(), "100000".to_string()], sixbit_chunks("1010101".to_string()));
    }

    #[test]
    fn test04_sixbit_chunks() {
        assert_eq!(vec!["100000".to_string()], sixbit_chunks("10".to_string()));
    }

    #[test]
    fn test05_sixbit_chunks() {
        let v_s: Vec<String> = Vec::new();
        assert_eq!(v_s, sixbit_chunks("".to_string()));
    }

    #[test]
    fn test01_character_bytes() {
        assert_eq!("01010100".to_string(), character_bytes('T'));
    }

    #[test]
    fn test02_character_bytes() {
        assert_eq!("1101000010100111".to_string(), character_bytes('Ч'));
    }

    #[test]
    fn test03_character_bytes() {
        assert_eq!("111000111000000110001010".to_string(), character_bytes('お'));
    }

    #[test]
    fn test01_string_characters() {
        assert_eq!(vec!['T','e','a',' ','i','s',' ','d','e','l','i','c','i','o','u','s','!'], string_characters("Tea is delicious!"));
    }

    #[test]
    fn test02_string_characters() {
        assert_eq!(vec!['Ч','а','й',' ','-',' ','э','т','о',' ','в','к','у','с','н','о','!'], string_characters("Чай - это вкусно!"));
    }

    #[test]
    fn test03_string_characters() {
        assert_eq!(vec!['お','茶','が','美','味','し','い','！'], string_characters("お茶が美味しい！"));
    }
}
