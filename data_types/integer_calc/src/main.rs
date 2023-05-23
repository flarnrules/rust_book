fn main() {

    let n: i128 = 64; // number of bits in an integer type
    let i_power: i128 = n - 1; // subtract 1 from number of bits for a signed integer
    let u_power: i128 = n; // don't subtract 1 from number of bits for an unsigned integer
    
    // set up i_exponent
    let i_exponent: i128 = 2;
    let i_exponent: i128 = i128::pow(i_exponent, i_power.try_into().unwrap());

    // set up u_exponent
    let u_exponent: i128 = 2;
    let u_exponent: i128 = i128::pow(u_exponent, u_power.try_into().unwrap());

    // set up signed integer range
    let begin_signed_range: i128 = -1 * i_exponent;
    
    println!("An i{n} integer starts with: {begin_signed_range}");

    let end_signed_range: i128 = i_exponent - 1;

    println!("An i{n} integer ends with: {end_signed_range}");

    // set up unsigned integer range

    let begin_unsigned_range: i128 = 0;

    println!("An u{n} integer starts with: {begin_unsigned_range}");

    let end_unsigned_range: i128 = u_exponent - 1;

    println!("An u{n} integer starts with: {end_unsigned_range}");
    
}