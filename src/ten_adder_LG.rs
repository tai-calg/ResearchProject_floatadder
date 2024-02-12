// ロジックゲートによるPrefix Adder

#![allow(non_snake_case)]


fn square_block_LG(Pi:bool, Pj:bool, Gi:bool, Gj:bool)->(bool,bool) {
    let Pij:bool = (Pi & Pj);
    let Gij:bool = ( (Pi & Gj) | Gi);
    return (Pij,Gij);
}

pub fn PA(p0:bool, p1:bool, p2:bool, p3:bool, p4:bool, p5:bool, p6:bool, p7:bool, p8:bool, p9:bool,
    g0:bool, g1:bool, g2:bool, g3:bool, g4:bool, g5:bool, g6:bool, g7:bool, g8:bool, g9:bool)
    ->(bool,bool,bool,bool,bool,  bool,bool,bool,bool,bool) {


    let (p_i1_d1, g_i1_d1) = square_block_LG(p0, p1, g0, g1);
    
    let (p_i2_d1, g_i2_d1) = square_block_LG(p_i1_d1, p2, g_i1_d1, g2);

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

    let (p_i8_d1, g_i8_d1) = square_block_LG(p_i7_d3, p8, g_i7_d3, g8);
    
    let (p_i9_d1, g_i9_d1) = square_block_LG(p8, p9, g8, g9);
    let (p_i9_d2, g_i9_d2) = square_block_LG(p_i7_d1, p_i9_d1, g_i7_d1, g_i9_d1);
    let (p_i9_d3, g_i9_d3) = square_block_LG(p_i5_d2, p_i9_d2, g_i5_d2, g_i9_d2);
    let (p_i9_d4, g_i9_d4) = square_block_LG(p_i1_d1, p_i9_d3, g_i1_d1, g_i9_d3);


    return (g0, g_i1_d1, g_i2_d1, g_i3_d2, g_i4_d1,
        g_i5_d3, g_i6_d1, g_i7_d3, g_i8_d1, g_i9_d4);
}

pub fn PGU(a0:bool, a1:bool, a2:bool, a3:bool, a4:bool, a5:bool, a6:bool, a7:bool, a8:bool, a9:bool,
    b0:bool, b1:bool, b2:bool, b3:bool, b4:bool, b5:bool, b6:bool, b7:bool, b8:bool, b9:bool)
    ->(bool,bool,bool,bool,bool,  bool,bool,bool,bool,bool) {

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

        return PA(p0,p1,p2,p3,p4,p5,p6,p7,p8,p9,
            g0,g1,g2,g3,g4,g5,g6,g7,g8,g9);
    }

    //Coutが怪しい
pub fn SGU(p0:bool, p1:bool, p2:bool, p3:bool, p4:bool, p5:bool, p6:bool, p7:bool, p8:bool, p9:bool,
    g00:bool, g10:bool ,g20:bool ,g30:bool ,g40:bool ,g50:bool ,g60:bool ,g70:bool ,g80:bool ,g90:bool, cin:bool)
    ->(bool, bool,bool,bool,bool,  bool,bool,bool,bool,bool, bool) {

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

        return (s0,s1,s2,s3,s4,s5,s6,s7,s8,s9,cout);
    }

    