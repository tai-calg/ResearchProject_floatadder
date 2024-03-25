// ロジックゲートによるPrefix Adder



#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn PA_interface_main(shifted_fr_a:u32, fract_b:u32 )-> u32{
    
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

                // ***TEST: 10bit Adder with cascading full adder*** //
            // let (s_list, c10) = cascade_FA(
            //     a_list[0], a_list[1], a_list[2], a_list[3], a_list[4], a_list[5], a_list[6], a_list[7], a_list[8], a_list[9],
            //     b_list[0], b_list[1], b_list[2], b_list[3], b_list[4], b_list[5], b_list[6], b_list[7], b_list[8], b_list[9],
            //     false
            // ); // これも正しく動く
                // ***   *** //


    let (p_list, g_list) = PGU(
        a_list[0], a_list[1], a_list[2], a_list[3], a_list[4], a_list[5], a_list[6], a_list[7], a_list[8], a_list[9],
        b_list[0], b_list[1], b_list[2], b_list[3], b_list[4], b_list[5], b_list[6], b_list[7], b_list[8], b_list[9]
    );

    let g_i0_list : [bool;10] = PA(
        p_list[0], p_list[1], p_list[2], p_list[3], p_list[4], p_list[5], p_list[6], p_list[7], p_list[8], p_list[9],
        g_list[0], g_list[1], g_list[2], g_list[3], g_list[4], g_list[5], g_list[6], g_list[7], g_list[8], g_list[9]
    );

    let (s_list,c10) = SGU(
        p_list[0], p_list[1], p_list[2], p_list[3], p_list[4], p_list[5], p_list[6], p_list[7], p_list[8], p_list[9],
        g_i0_list[0], g_i0_list[1], g_i0_list[2], g_i0_list[3], g_i0_list[4], g_i0_list[5], g_i0_list[6], g_i0_list[7], g_i0_list[8], g_i0_list[9],
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

fn square_block_LG(Pi:bool, Pj:bool, Gi:bool, Gj:bool)->(bool,bool) {
    let Pij:bool = Pi & Pj;
    let Gij:bool =  (Pj & Gi) | Gj ;
    return (Pij,Gij);
}

pub fn PA(p0:bool, p1:bool, p2:bool, p3:bool, p4:bool, p5:bool, p6:bool, p7:bool, p8:bool, p9:bool,
    g0:bool, g1:bool, g2:bool, g3:bool, g4:bool, g5:bool, g6:bool, g7:bool, g8:bool, g9:bool)
    ->[bool;10] {


    let (p_i1_d1, g_i1_d1) = square_block_LG(p0, p1, g0, g1);
    
    let (p_i2_d1, g_i2_d1) = square_block_LG(p_i1_d1, p2, g_i1_d1, g2);
    assert!( g_i2_d1 == ( g2 | (p2&(g1|(p1&g0)) ) ));

    let (p_i3_d1, g_i3_d1) = square_block_LG(p2, p3, g2, g3);
    let (p_i3_d2, g_i3_d2) = square_block_LG(p_i1_d1, p_i3_d1, g_i1_d1, g_i3_d1);

    let (p_i4_d1, g_i4_d1) = square_block_LG(p_i3_d2, p4, g_i3_d2, g4);

    let (p_i5_d1, g_i5_d1) = square_block_LG(p4, p5, g4, g5);
    let (p_i5_d2, g_i5_d2) = square_block_LG(p_i3_d1, p_i5_d1, g_i3_d1, g_i5_d1);
    let (p_i5_d3, g_i5_d3) = square_block_LG(p_i1_d1, p_i5_d2, g_i1_d1, g_i5_d2);

    let (p_i6_d1, g_i6_d1) = square_block_LG(p_i5_d3, p6, g_i5_d3, g6);

    let (p_i7_d1, g_i7_d1) = square_block_LG(p6, p7, g6, g7);
    let (p_i7_d2, g_i7_d2) = square_block_LG(p_i5_d1, p_i7_d1, g_i5_d1, g_i7_d1);
    let (p_i7_d3, g_i7_d3) = square_block_LG(p_i3_d2, p_i7_d2, g_i3_d2, g_i7_d2);
    assert!( p_i7_d3== (p0&p1&p2&p3&p4&p5&p6&p7));

    let (p_i8_d1, g_i8_d1) = square_block_LG(p_i7_d3, p8, g_i7_d3, g8);
    
    let (p_i9_d1, g_i9_d1) = square_block_LG(p8, p9, g8, g9);
    let (p_i9_d2, g_i9_d2) = square_block_LG(p_i7_d1, p_i9_d1, g_i7_d1, g_i9_d1);
    let (p_i9_d3, g_i9_d3) = square_block_LG(p_i5_d2, p_i9_d2, g_i5_d2, g_i9_d2);
    let (p_i9_d4, g_i9_d4) = square_block_LG(p_i1_d1, p_i9_d3, g_i1_d1, g_i9_d3);
    assert!( p_i9_d4== (p0&p1&p2&p3&p4&p5&p6&p7&p8&p9));
    assert!( g_i9_d4 == ( g9 | (g8&p9) | (g7&p8&p9) | (g6&p7&p8&p9) | (g5&p6&p7&p8&p9)| (g4&p5&p6&p7&p8&p9) |
    (g3&p4&p5&p6&p7&p8&p9) | (g2&p3&p4&p5&p6&p7&p8&p9) | (g1&p2&p3&p4&p5&p6&p7&p8&p9) | (g0&p1&p2&p3&p4&p5&p6&p7&p8&p9) ) );


    return [g0, g_i1_d1, g_i2_d1, g_i3_d2, g_i4_d1,
        g_i5_d3, g_i6_d1, g_i7_d3, g_i8_d1, g_i9_d4];
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

        // let s0 = cin ^ p0;
        let s0 =  p0;
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

    
    // Adder 検証用のFullAdder
fn full_adder(a:bool, b:bool, cin:bool)->(bool, bool) {
    let sum = a ^ b ^ cin;
    let cout = (a ^ b) & cin | a & b;
    return (sum,cout);
}

fn cascade_FA(a0:bool, a1:bool, a2:bool, a3:bool , a4:bool, a5:bool, a6:bool, a7:bool, a8:bool, a9:bool,
    b0:bool, b1:bool, b2:bool, b3:bool , b4:bool, b5:bool, b6:bool, b7:bool, b8:bool, b9:bool,
    cin:bool)->([bool;10], bool){

    let (s0,c0) = full_adder(a0,b0,cin);
    let (s1,c1) = full_adder(a1,b1,c0);
    let (s2,c2) = full_adder(a2,b2,c1);
    let (s3,c3) = full_adder(a3,b3,c2);
    let (s4,c4) = full_adder(a4,b4,c3);
    let (s5,c5) = full_adder(a5,b5,c4);
    let (s6,c6) = full_adder(a6,b6,c5);
    let (s7,c7) = full_adder(a7,b7,c6);
    let (s8,c8) = full_adder(a8,b8,c7);
    let (s9,cout) = full_adder(a9,b9,c8);

        
    return ([s0,s1,s2,s3,s4,s5,s6,s7,s8,s9],cout);

}


// ===== //



pub fn PA_interface_main_with_cin(shifted_fr_a:u32, fract_b:u32, cin : bool )-> u32{
    
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

                // ***TEST: 10bit Adder with cascading full adder*** //
            // let (s_list, c10) = cascade_FA(
            //     a_list[0], a_list[1], a_list[2], a_list[3], a_list[4], a_list[5], a_list[6], a_list[7], a_list[8], a_list[9],
            //     b_list[0], b_list[1], b_list[2], b_list[3], b_list[4], b_list[5], b_list[6], b_list[7], b_list[8], b_list[9],
            //     false
            // ); // これも正しく動く
                // ***   *** //


    let (p_list, g_list) = PGU(
        a_list[0], a_list[1], a_list[2], a_list[3], a_list[4], a_list[5], a_list[6], a_list[7], a_list[8], a_list[9],
        b_list[0], b_list[1], b_list[2], b_list[3], b_list[4], b_list[5], b_list[6], b_list[7], b_list[8], b_list[9]
    );

    let c_list : [bool;10] = PA_with_cin(
        p_list[0], p_list[1], p_list[2], p_list[3], p_list[4], p_list[5], p_list[6], p_list[7], p_list[8], p_list[9],
        g_list[0], g_list[1], g_list[2], g_list[3], g_list[4], g_list[5], g_list[6], g_list[7], g_list[8], g_list[9] ,cin
    );

    let (s_list,c10) = SGU(
        p_list[0], p_list[1], p_list[2], p_list[3], p_list[4], p_list[5], p_list[6], p_list[7], p_list[8], p_list[9],
        c_list[0], c_list[1], c_list[2], c_list[3], c_list[4], c_list[5], c_list[6], c_list[7], c_list[8], c_list[9],
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

pub fn PA_with_cin(p0:bool, p1:bool, p2:bool, p3:bool, p4:bool, p5:bool, p6:bool, p7:bool, p8:bool, p9:bool,
    g0:bool, g1:bool, g2:bool, g3:bool, g4:bool, g5:bool, g6:bool, g7:bool, g8:bool, g9:bool, cin:bool)
    ->[bool;10] {


    let (p_i1_d1, g_i1_d1) = square_block_LG(p0, p1, g0, g1);
    
    let (p_i2_d1, g_i2_d1) = square_block_LG(p_i1_d1, p2, g_i1_d1, g2);
    assert!( g_i2_d1 == ( g2 | (p2&(g1|(p1&g0)) ) ));

    let (p_i3_d1, g_i3_d1) = square_block_LG(p2, p3, g2, g3);
    let (p_i3_d2, g_i3_d2) = square_block_LG(p_i1_d1, p_i3_d1, g_i1_d1, g_i3_d1);

    let (p_i4_d1, g_i4_d1) = square_block_LG(p_i3_d2, p4, g_i3_d2, g4);

    let (p_i5_d1, g_i5_d1) = square_block_LG(p4, p5, g4, g5);
    let (p_i5_d2, g_i5_d2) = square_block_LG(p_i3_d1, p_i5_d1, g_i3_d1, g_i5_d1);
    let (p_i5_d3, g_i5_d3) = square_block_LG(p_i1_d1, p_i5_d2, g_i1_d1, g_i5_d2);

    let (p_i6_d1, g_i6_d1) = square_block_LG(p_i5_d3, p6, g_i5_d3, g6);

    let (p_i7_d1, g_i7_d1) = square_block_LG(p6, p7, g6, g7);
    let (p_i7_d2, g_i7_d2) = square_block_LG(p_i5_d1, p_i7_d1, g_i5_d1, g_i7_d1);
    let (p_i7_d3, g_i7_d3) = square_block_LG(p_i3_d2, p_i7_d2, g_i3_d2, g_i7_d2);
    assert!( p_i7_d3== (p0&p1&p2&p3&p4&p5&p6&p7));

    let (p_i8_d1, g_i8_d1) = square_block_LG(p_i7_d3, p8, g_i7_d3, g8);
    
    let (p_i9_d1, g_i9_d1) = square_block_LG(p8, p9, g8, g9);
    let (p_i9_d2, g_i9_d2) = square_block_LG(p_i7_d1, p_i9_d1, g_i7_d1, g_i9_d1);
    let (p_i9_d3, g_i9_d3) = square_block_LG(p_i5_d2, p_i9_d2, g_i5_d2, g_i9_d2);
    let (p_i9_d4, g_i9_d4) = square_block_LG(p_i1_d1, p_i9_d3, g_i1_d1, g_i9_d3);
    assert!( p_i9_d4== (p0&p1&p2&p3&p4&p5&p6&p7&p8&p9));
    assert!( g_i9_d4 == ( g9 | (g8&p9) | (g7&p8&p9) | (g6&p7&p8&p9) | (g5&p6&p7&p8&p9)| (g4&p5&p6&p7&p8&p9) |
    (g3&p4&p5&p6&p7&p8&p9) | (g2&p3&p4&p5&p6&p7&p8&p9) | (g1&p2&p3&p4&p5&p6&p7&p8&p9) | (g0&p1&p2&p3&p4&p5&p6&p7&p8&p9) ) );


    // OUTPUT is Carry
        let c0 = ( p0 & cin )  | g0;
        let c1 = ( p_i1_d1 & c0 ) | g_i1_d1;
        let c2 = ( p_i2_d1 & c1 ) | g_i2_d1;
        let c3 = ( p_i3_d2 & c2 ) | g_i3_d2;
        let c4 = ( p_i4_d1 & c3 ) | g_i4_d1;
        let c5 = ( p_i5_d3 & c4 ) | g_i5_d3;
        let c6 = ( p_i6_d1 & c5 ) | g_i6_d1;
        let c7 = ( p_i7_d3 & c6 ) | g_i7_d3;
        let c8 = ( p_i8_d1 & c7 ) | g_i8_d1;
        let c9 = ( p_i9_d4 & c8 ) | g_i9_d4;

    return [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9];
}