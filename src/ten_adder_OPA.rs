#![allow(non_snake_case)]
#![allow(dead_code)]



#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn OPA_interface_main(shifted_fr_a:u32, fract_b:u32)-> u32{
    
    /*    
        addsub_result = (shifted_fr_a + fract_b) & 0b01_1111_1111; //9bit, because 10bit as sign bit must be 0. 
    */

    // fract_b :10bit , 001 | frb . shifted_fr_a : 10bit , 001 | fra OR ( (110 | !fra ) +1 ) >> shift_val
    let b_list : [bool;10] = [
        (fract_b & 0b00_0000_0001) != 0,
        (fract_b & 0b00_0000_0010) != 0,
        (fract_b & 0b00_0000_0100) != 0,
        (fract_b & 0b00_0000_1000) != 0,
        (fract_b & 0b00_0001_0000) != 0,
        (fract_b & 0b00_0010_0000) != 0,
        (fract_b & 0b00_0100_0000) != 0,
        (fract_b & 0b00_1000_0000) != 0,
        (fract_b & 0b01_0000_0000) != 0,
        (fract_b & 0b10_0000_0000) != 0,
    ];
    let a_list : [bool;10] = [
        (shifted_fr_a & 0b00_0000_0001) != 0,
        (shifted_fr_a & 0b00_0000_0010) != 0,
        (shifted_fr_a & 0b00_0000_0100) != 0,
        (shifted_fr_a & 0b00_0000_1000) != 0,
        (shifted_fr_a & 0b00_0001_0000) != 0,
        (shifted_fr_a & 0b00_0010_0000) != 0,
        (shifted_fr_a & 0b00_0100_0000) != 0,
        (shifted_fr_a & 0b00_1000_0000) != 0,
        (shifted_fr_a & 0b01_0000_0000) != 0,
        (shifted_fr_a & 0b10_0000_0000) != 0,
    ];




    let (p_list, g_list) = PGU(
        a_list[0], a_list[1], a_list[2], a_list[3], a_list[4], a_list[5], a_list[6], a_list[7], a_list[8], a_list[9],
        b_list[0], b_list[1], b_list[2], b_list[3], b_list[4], b_list[5], b_list[6], b_list[7], b_list[8], b_list[9]
    );

    let p_list_f64 = p_list.iter().map(|x| if *x {1.0} else {0.0}).collect::<Vec<f64>>();
    let g_list_f64 = g_list.iter().map(|x| if *x {1.0} else {0.0}).collect::<Vec<f64>>();
    
    let g_i0_list : [f64;10] = OPA(
        p_list_f64[0], p_list_f64[1], p_list_f64[2], p_list_f64[3], p_list_f64[4], p_list_f64[5], p_list_f64[6], p_list_f64[7], p_list_f64[8], p_list_f64[9],
        g_list_f64[0], g_list_f64[1], g_list_f64[2], g_list_f64[3], g_list_f64[4], g_list_f64[5], g_list_f64[6], g_list_f64[7], g_list_f64[8], g_list_f64[9]
    );
    
    // *** ADC_1 process *** //
    // ADCの閾値は0.2くらいが最適？それでも8442/50000のエラーが出る．
    // for g_i0_list.map(|x| if x>=0.1 {1} else {0})
    let g_i0_list_bool=g_i0_list.iter().map(|x| if *x > 0.1 {true} else {false}).collect::<Vec<bool>>();


    let (s_list,c10) = SGU(
        p_list[0], p_list[1], p_list[2], p_list[3], p_list[4], p_list[5], p_list[6], p_list[7], p_list[8], p_list[9],
        g_i0_list_bool[0], g_i0_list_bool[1], g_i0_list_bool[2], g_i0_list_bool[3], g_i0_list_bool[4], g_i0_list_bool[5], g_i0_list_bool[6], g_i0_list_bool[7], g_i0_list_bool[8], g_i0_list_bool[9],
        false
    );
    
    //どのみちcout(:c10)は捨てる


    let result:u32 = 
        // (if s_list[9] {0b10_0000_0000} else {0}) | // ここは無くてもいい
        (if s_list[8] {0b01_0000_0000} else {0}) |
        (if s_list[7] {0b00_1000_0000} else {0}) |
        (if s_list[6] {0b00_0100_0000} else {0}) |
        (if s_list[5] {0b00_0010_0000} else {0}) |
        (if s_list[4] {0b00_0001_0000} else {0}) |
        (if s_list[3] {0b00_0000_1000} else {0}) |
        (if s_list[2] {0b00_0000_0100} else {0}) |
        (if s_list[1] {0b00_0000_0010} else {0}) |
        (if s_list[0] {0b00_0000_0001} else {0}) ;

        return result; 

}

//ref power point is "1220_2023_Adderの提案整理.pptx"
fn psi_and(pa:f64,pb:f64)->f64 {
    let p1 = 9.0_f64 * 0.38_f64 / 4.0_f64;
    
    let pout:f64 =  0.38_f64 *( (pa).sqrt()+(pb).sqrt() - (1.0_f64/ 4.0_f64).sqrt() ) 
    .powf(2.0); // - (p1/ 4.0_f64).sqrt() vs - (1.0_f64/ 4.0_f64).sqrt() とどっちが正しい？... 1.0の方が(1,0), (0,0)のOUTPUT強度が一致するので正解．
    return pout;
}

fn dc_or(pa:f64,pb:f64)->f64 {
    
    let pout:f64 =  0.5_f64 * (
         pa + pb + 2.0_f64*(pa*pb).sqrt() 
        ); 
    return pout;
}

fn square_block(pi:f64, pj:f64, gi:f64, gj:f64)->(f64,f64) {
    let pij = psi_and(pi/2.0,pj);
    let gij = dc_or(psi_and(pj/2.0, gi), gj);
    return (pij,gij);
}



pub fn OPA(p0:f64, p1:f64, p2:f64, p3:f64, p4:f64, p5:f64, p6:f64, p7:f64, p8:f64, p9:f64,
    g0:f64, g1:f64, g2:f64, g3:f64, g4:f64, g5:f64, g6:f64, g7:f64, g8:f64, g9:f64)
    -> [f64;10] {


    let (p_i1_d1, g_i1_d1) = square_block(p0/2.0, p1, g0/2.0, g1);
    
    let (p_i2_d1, g_i2_d1) = square_block(p_i1_d1/16.0, p2/2.0, g_i1_d1/16.0, g2/2.0);

    let (p_i3_d1, g_i3_d1) = square_block(p2/2.0, p3, g2/2.0, g3);
    let (p_i3_d2, g_i3_d2) = square_block(p_i1_d1/2.0, p_i3_d1/2.0, g_i1_d1/2.0, g_i3_d1/2.0);

    let (p_i4_d1, g_i4_d1) = square_block(p_i3_d2/2.0, p4/2.0, g_i3_d2/2.0, g4/2.0);

    let (p_i5_d1, g_i5_d1) = square_block(p4/2.0, p5, g4/2.0, g5);
    let (p_i5_d2, g_i5_d2) = square_block(p_i3_d1/2.0, p_i5_d1/2.0, g_i3_d1/2.0, g_i5_d1/2.0);
    let (p_i5_d3, g_i5_d3) = square_block(p_i1_d1/4.0, p_i5_d2/2.0, g_i1_d1/4.0, g_i5_d2/2.0);

    let (p_i6_d1, g_i6_d1) = square_block(p_i5_d3/2.0, p6/2.0, g_i5_d3/2.0, g6/2.0);

    let (p_i7_d1, g_i7_d1) = square_block(p6/2.0, p7, g6/2.0, g7);
    let (p_i7_d2, g_i7_d2) = square_block(p_i5_d1/2.0, p_i7_d1/2.0, g_i5_d1/2.0, g_i7_d1/2.0);
    let (p_i7_d3, g_i7_d3) = square_block(p_i3_d2/2.0, p_i7_d2, g_i3_d2/2.0, g_i7_d2);

    let (p_i8_d1, g_i8_d1) = square_block(p_i7_d3/2.0, p8/2.0, g_i7_d3/2.0, g8/2.0);
    
    let (p_i9_d1, g_i9_d1) = square_block(p8/2.0, p9, g8/2.0, g9);
    let (p_i9_d2, g_i9_d2) = square_block(p_i7_d1/2.0, p_i9_d1, g_i7_d1/2.0, g_i9_d1);
    let (p_i9_d3, g_i9_d3) = square_block(p_i5_d2/2.0, p_i9_d2, g_i5_d2/2.0, g_i9_d2);
    let (p_i9_d4, g_i9_d4) = square_block(p_i1_d1/8.0, p_i9_d3, g_i1_d1/8.0, g_i9_d3);


    // return (g0/2.0, g_i1_d1/16.0, g_i2_d1, g_i3_d2/4.0, g_i4_d1,
        // g_i5_d3/2.0, g_i6_d1, g_i7_d3/2.0, g_i8_d1, g_i9_d4);
    return [g0/2.0, g_i1_d1/16.0, g_i2_d1, g_i3_d2/4.0, g_i4_d1,
        g_i5_d3/2.0, g_i6_d1, g_i7_d3/2.0, g_i8_d1, g_i9_d4];
}

pub fn PGU(a0:bool, a1:bool, a2:bool, a3:bool, a4:bool, a5:bool, a6:bool, a7:bool, a8:bool, a9:bool,
    b0:bool, b1:bool, b2:bool, b3:bool, b4:bool, b5:bool, b6:bool, b7:bool, b8:bool, b9:bool)
    ->([bool;10], [bool;10]) {

        let (p0,g0) = (a0^b0, a0&b0);
        let (p1,g1) = (a1^b1, a1&b1);
        let (p2,g2) = (a2^b2, a2&b2);
        let (p3,g3) = (a3^b3, a3&b3);
        let (p4,g4) = (a4^b4, a4&b4);
        let (p5,g5) = (a5^b5, a5&b5);
        let (p6,g6) = (a6^b6, a6&b6);
        let (p7,g7) = (a7^b7, a7&b7);
        let (p8,g8) = (a8^b8, a8&b8);
        let (p9,g9) = (a9^b9, a9&b9);

        return ([p0,p1,p2,p3,p4,p5,p6,p7,p8,p9],
            [g0,g1,g2,g3,g4,g5,g6,g7,g8,g9]);
    }


pub fn SGU(p0:bool, p1:bool, p2:bool, p3:bool, p4:bool, p5:bool, p6:bool, p7:bool, p8:bool, p9:bool,
    g00:bool, g10:bool ,g20:bool ,g30:bool ,g40:bool ,g50:bool ,g60:bool ,g70:bool ,g80:bool ,g90:bool, cin:bool)
    ->([bool;10], bool) {

        let s0 = cin ^ p0;
        let s1 = g00 ^ p1;
        let s2 = g10 ^ p2;
        let s3 = g20 ^ p3;
        let s4 = g30 ^ p4;
        let s5 = g40 ^ p5;
        let s6 = g50 ^ p6;
        let s7 = g60 ^ p7;
        let s8 = g70 ^ p8;
        let s9 = g80 ^ p9;
        let cout = g90;

        return ([s0,s1,s2,s3,s4,s5,s6,s7,s8,s9],cout);
    }


// *** without Y-branch attenuation  VERSION ***  //

fn square_block_without_YBranch_attenuation(Pi:f64, Pj:f64, Gi:f64, Gj:f64)->(f64,f64) {
    let Pij = psi_and(Pi,Pj);
    let Gij = dc_or(psi_and(Pj, Gi), Gj);
    return (Pij,Gij);
}

pub fn OPA_without_YBranch_attenuation(p0:f64, p1:f64, p2:f64, p3:f64, p4:f64, p5:f64, p6:f64, p7:f64, p8:f64, p9:f64,
    g0:f64, g1:f64, g2:f64, g3:f64, g4:f64, g5:f64, g6:f64, g7:f64, g8:f64, g9:f64)
    ->[f64;10] {


    let (p_i1_d1, g_i1_d1) = square_block_without_YBranch_attenuation(p0, p1, g0, g1);
    
    let (p_i2_d1, g_i2_d1) = square_block_without_YBranch_attenuation(p_i1_d1, p2, g_i1_d1, g2);

    let (p_i3_d1, g_i3_d1) = square_block_without_YBranch_attenuation(p2, p3, g2, g3);
    let (p_i3_d2, g_i3_d2) = square_block_without_YBranch_attenuation(p_i1_d1, p_i3_d1, g_i1_d1, g_i3_d1);

    let (p_i4_d1, g_i4_d1) = square_block_without_YBranch_attenuation(p_i3_d2, p4, g_i3_d2, g4);

    let (p_i5_d1, g_i5_d1) = square_block_without_YBranch_attenuation(p4, p5, g4, g5);
    let (p_i5_d2, g_i5_d2) = square_block_without_YBranch_attenuation(p_i3_d1, p_i5_d1, g_i3_d1, g_i5_d1);
    let (p_i5_d3, g_i5_d3) = square_block_without_YBranch_attenuation(p_i1_d1, p_i5_d2, g_i1_d1, g_i5_d2);

    let (p_i6_d1, g_i6_d1) = square_block_without_YBranch_attenuation(p_i5_d3, p6, g_i5_d3, g6);

    let (p_i7_d1, g_i7_d1) = square_block_without_YBranch_attenuation(p6, p7, g6, g7);
    let (p_i7_d2, g_i7_d2) = square_block_without_YBranch_attenuation(p_i5_d1, p_i7_d1, g_i5_d1, g_i7_d1);
    let (p_i7_d3, g_i7_d3) = square_block_without_YBranch_attenuation(p_i3_d2, p_i7_d2, g_i3_d2, g_i7_d2);

    let (p_i8_d1, g_i8_d1) = square_block_without_YBranch_attenuation(p_i7_d3, p8, g_i7_d3, g8);
    
    let (p_i9_d1, g_i9_d1) = square_block_without_YBranch_attenuation(p8, p9, g8, g9);
    let (p_i9_d2, g_i9_d2) = square_block_without_YBranch_attenuation(p_i7_d1, p_i9_d1, g_i7_d1, g_i9_d1);
    let (p_i9_d3, g_i9_d3) = square_block_without_YBranch_attenuation(p_i5_d2, p_i9_d2, g_i5_d2, g_i9_d2);
    let (p_i9_d4, g_i9_d4) = square_block_without_YBranch_attenuation(p_i1_d1, p_i9_d3, g_i1_d1, g_i9_d3);


    return [g0, g_i1_d1, g_i2_d1, g_i3_d2, g_i4_d1,
        g_i5_d3, g_i6_d1, g_i7_d3, g_i8_d1, g_i9_d4];
}

// fn SGU 

