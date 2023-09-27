use std::cmp;
use std::fs;

/*
    let input1:u32 = 0b1100_0000_1000_0101; // bfloat16 
    let input2:u32 = 0b0000_0110_1000_0011; // bfloat16
    の16bitをランダム生成にして、10000回ほどループで回して,それをassert_eq!で比較する.
*/
fn main() {
    // let input1:u32 = 0b1_01111110_0000101; // bfloat16 
    // let input2:u32 = 0b0_01111110_0000011; // bfloat16 
    let contents = fs::read_to_string("src/w_value_bin.txt")
        .expect("Something went wrong reading the file");

    // 文字列の前後の空白を削除し、カンマで分割する
    let bin_values: Vec<u32> = contents.trim().split(',')
    .filter_map(|s| u32::from_str_radix(s, 2).ok())
    .collect();
    assert_eq!(0b0000_0000_0111_1111 , u32::from_str_radix("0000000001111111", 2).unwrap());

    // println!("### float adder ###"); //res: 0.015625 * 2^(0) = 0.015625 //下駄によりexpは-126 する。


    //// for i ,i+1 ; 0 ~ bin_values.len() - 1 ; i+=2; bin_values[i] , bin_values[i+1]
    let mut output_list = Vec::new();
    for i in (0..bin_values.len()).step_by(2) {
        // println!("### float adder ###"); //res: 0.015625 * 2^(0) = 0.015625 //下駄によりexpは-126 する。
        // println!("input1: {:0>16b}", bin_values[i]);
        // println!("input2: {:0>16b}", bin_values[i+1]);
        let output = float_adder_run(bin_values[i], bin_values[i+1]);
        output_list.push(output);

    }

    //// write to file as segment is ","
    let mut output_str = String::new();
    for i in 0..output_list.len() {
        output_str.push_str(&format!("{:0>16b}", output_list[i]));
        if i != output_list.len() - 1 {
            output_str.push_str(",");
        }
    }
    fs::write("src/w_self_add_result.txt", output_str).expect("Unable to write file");



    // println!("### float adder by ieee ###");
    // let ieeef32_1 = ieee_to_f32( bfloat16_to_ieee(input1 as u16) );
    // let ieeef32_2 = ieee_to_f32( bfloat16_to_ieee(input2 as u16) );
    // println!("ieeef32_1: {}", ieeef32_1);
    // println!("ieeef32_2: {}", ieeef32_2);
    // let res_f32 = ieeef32_1 + ieeef32_2;
    // let res_u32 = f32_to_ieee(res_f32);
    // println!("res_f32: {}", res_f32);
    // println!("res_u32: {:0>32b}",  res_u32);
    // assert_eq!(output, res_u32 >> 16);
}

fn float_adder_run(input1:u32, input2:u32)-> u32 {

// procedual 1 : swap 
    let sign_mask = 0b1000_0000_0000_0000; // [15]
    let in_sign1:bool = (input1 & sign_mask) != 0;
    let in_sign2:bool = (input2 & sign_mask) != 0;

    let exp_mask = 0b0111_1111_1000_0000; // [14:7]
    let in_exp1 = (input1 & exp_mask) >> 7; // 7bit右シフトして範囲を0~255にする
    let in_exp2 = (input2 & exp_mask) >> 7; 
    
    let fract_mask = 0b0000_0000_0111_1111; // [6:0]
    
    let in_fract1 = input1 & fract_mask;
    let in_fract2 = input2 & fract_mask;

    let mut sign_a = false; // false: +, true: -
    let mut exp_a = 0;
    let mut fract_a = 0;

    let mut sign_b = false;
    let mut exp_b = 0;
    let mut fract_b = 0;

    let input1_expfr = input1 & 0b0111_1111_1111_1111 ;
    let input2_expfr = input2 & 0b0111_1111_1111_1111 ;
    if input1_expfr > input2_expfr {
        sign_b = in_sign1;
        exp_b = in_exp1;
        fract_b = in_fract1;

        sign_a = in_sign2;
        exp_a = in_exp2;
        fract_a = in_fract2;

    }else {
        sign_b = in_sign2;
        exp_b = in_exp2;
        fract_b = in_fract2;
        
        sign_a = in_sign1;
        exp_a = in_exp1;
        fract_a = in_fract1;
    }
        // MUST b > a

// procedual 2 : shift
    let shift_val = cmp::min(exp_b - exp_a,8);
    let exp_ep = exp_b - exp_a;
    fract_a |= 0b0000_0000_1000_0000; //hidden bitを結合
    fract_b |= 0b0000_0000_1000_0000; //hidden bitを結合

    let overflow_mask:u32 = (1 << shift_val) - 1;
    let overflowed_bits:u32 = fract_a & overflow_mask;
    let mut guard_bit : bool = false;
    if shift_val >= 1 {
        guard_bit = (overflowed_bits >> (shift_val-1) ) & 0b0001 == 1;
    }
    let mut round_bit : bool = false;
    let mut sticky_mask:u32 = 0;
    if shift_val >= 2 {
        round_bit = (overflowed_bits >> (shift_val-2) ) & 0b0001 == 1;
        sticky_mask = (1 << (shift_val-2)) - 1;
    }
    let sticky_bit : bool = (overflowed_bits & sticky_mask) != 0b0000_00;
    // !!! //
    //@ GRS roundはshifted_fract_aに対して行う！
    let mut shifted_fract_a = fract_a >> shift_val;

    // procedual 5←変更 : round

    let mut fract_rou = fract_a & 0b0000_0000_0111_1111; // 7bit
    // sgn_a and sgn_b 

    let fr_all1:bool = fract_rou == 0b0000_0000_0111_1111;
    let ulp:bool = (fract_rou & 0b0000_0000_0000_0001) == 0b1;


    if exp_ep <= 8 {
        if guard_bit & (sticky_bit | round_bit | ulp) {
            shifted_fract_a += 1;
        }
    }

// procedual 3 : add , sub 
    let add_result = fract_b + shifted_fract_a; // In:8bit, Out:桁上がりの含めて9bit
    let sub_result = fract_b - shifted_fract_a; // 桁上がりの含めて9bit
    
    // if shifted_fract_a != 0 { //[test]
    //     println!("add_result: {:0>16b}", add_result);
    //     println!("sub_result: {:0>16b}", sub_result);
    //     println!("fract_b   : {:0>16b}", fract_b);
    //     println!("----------");
    // }

    // println!("add_result: {}", add_result);
    // println!("sub_result: {:0>16b}", sub_result);
        
    //xnor sign for mux selector
    let selector = !(sign_a ^ sign_b);
    let calc_result = if selector {add_result} else {sub_result};

// procedual 4 : normalize
    /*
    Addを選択した時…  if (    fr[8](...最上位ビットである9番目) == 1    ){exp++; fr >>1;}else{} fractはinput[6:0]を出力
    Subを選択した時…  while (   fr[7](...切り捨てる予定の整数部) == 0    ) { exp--; fr << 1;} while後にinput[6:0]を出力
     */
    let mut exp = exp_b;
    let mut fract = calc_result; //9bit

    if selector { // add
        if (fract & 0b0000_0001_0000_0000) != 0 { // 9bit目が1の時(桁あがり)
            exp += 1;
            fract = fract >> 1;
        }else{
            // do nothing
        }
    }else { // sub
        while(fract & 0b0000_0000_1000_0000) == 0 { // 8bit目が0の時(切り捨てる予定の整数部)
            exp -= 1; // max -7
            fract = fract << 1;
        }
    }



// ### //


// procedual 6 : 例外処理
/*
val    s_exponent_signcnd
+inf = 0_11111111_0000000
-inf = 1_11111111_0000000

val    s_exponent_signcnd
+NaN = 0_11111111_{not all 0}
-NaN = 1_11111111_{not all 0}

0    = 0 00000000 0000000 = 0
0    = 1 00000000 0000000 = −0

*/
let exp_result: u32 = exp ;
let fract_result = fract & 0b0000_0000_0111_1111; // 7bit
let sign_result = sign_b;

if exp_result == 0b11111111 {
        if fract_result == 0 {
            // +inf or -inf
            return (sign_result as u32) << 15 | 0b0_11111111_0000000;
        }else{
            // NaN
            // tf ではNanは fract is {all 1}である。
            return 0b011111111_1111111;
        }
    }

// procedual 7 : binding bits process 



    //bind sgn | exp | fract
    let result = (sign_result as u32) << 15 | (exp_result << 7) | fract_result;


    return result; 

}

fn u32_to_bool_array(n: u32) -> [bool; 32] {
    let mut array = [false; 32];
    for i in 0..32 {
        array[i] = ((n >> i) & 1) == 1;
    }
    array
}

fn bfloat16_to_ieee(bf16: u16) -> u32 {
    let sign = (bf16 & 0b1000_0000_0000_0000) as u32;
    let exp = (bf16 & 0b0111_1111_1000_0000) as u32;
    let fract = (bf16 & 0b0000_0000_0111_1111) as u32;

    (sign << 16) | (exp << 16) | (fract << 16)
}

fn ieee_to_f32(ieee: u32) -> f32 {
    f32::from_bits(ieee)
}

fn f32_to_ieee(f32: f32) -> u32 {
    f32.to_bits()
}