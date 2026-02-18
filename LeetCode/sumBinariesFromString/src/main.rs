fn sum_binaries_from_string(a: &str, b: &str) -> Result<String, String> {
    let mut result = String::new();

    if a.len() < 1 && b.len() > 10_usize.pow(4) { return Err("Invalid input".into()); }

    let max_len = a.len().max(b.len());

    let a_padded = format!("{:0>width$}", a, width = max_len);
    let b_padded = format!("{:0>width$}", b, width = max_len);

    let mut left: bool = false;
    for (value_a, value_b) in a_padded.chars().rev().zip(b_padded.chars().rev()) {

        let bit_a: bool = value_a == '1';
        let bit_b: bool = value_b == '1';
        let bit: bool = match left {
            true  => bit_a == bit_b,
            false => bit_a ^ bit_b,
        };

        result.push(if bit { '1' } else { '0' });

        // println!("{} {} {} {}", left, value_a, value_b, bit);

        left = if left { bit_a || bit_b } else { bit_a && bit_b };

    }
    if left {
        result.push('1');
    }
    
    Ok(result.chars().rev().collect())
}

fn main() {
    let a = "1110";
    let b = "0011";
    let result: String = sum_binaries_from_string(&a,&b).unwrap();
    println!("a {} ", a);
    println!("b {} ", b);
    println!("-----------");
    println!(" {}", result);
}