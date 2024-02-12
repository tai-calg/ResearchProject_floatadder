#![allow(non_snake_case)]


/// 前提：
/// Pbiasは動的に決定．（元論文ではPbiasは静的で固定）→gi , piは別々のAND,ORが必要！光デバイスの消費エネルギー二倍！
/// 動的にすることで(0,0)の時0.097ではなく，0になる．
/// 動的にする手法：OEO．100:1くらいの比で光をちょっと受け取ってそれを電気にしたのちまたレーザーを使ってOにする．


// ignore snake_case
//ref power point is "1220_2023_Adderの提案整理.pptx"
fn psi_and(Pa:f64,Pb:f64)->f64 {
    let Pout:f64 = ( (Pa.sqrt() + 2.0*Pb.sqrt() )/2.0*
    0.38_f64.sqrt() ).powf(2.0);
    return Pout;
}

// directional_coupler_or
fn dc_or(Pa:f64,Pb:f64)->f64 {
    // let P_out_under = (Pa-Pb).powf(2.0);
    return (Pa+Pb).powf(2.0);
}

fn square_block(Pi:f64, Pj:f64, Gi:f64, Gj:f64)->(f64,f64) {
    let Pij = psi_and(Pi/2.0,Pj);
    let Gij = dc_or(psi_and(Pi/2.0, Gj), Gi);
    return (Pij,Gij);
}



pub fn OPA(p0:f64, p1:f64, p2:f64, p3:f64, p4:f64, p5:f64, p6:f64, p7:f64, p8:f64, p9:f64,
    g0:f64, g1:f64, g2:f64, g3:f64, g4:f64, g5:f64, g6:f64, g7:f64, g8:f64, g9:f64)
    ->(f64,f64,f64,f64,f64,  f64,f64,f64,f64,f64) {


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


    return (g0/2.0, g_i1_d1/16.0, g_i2_d1, g_i3_d2/4.0, g_i4_d1,
        g_i5_d3/2.0, g_i6_d1, g_i7_d3/2.0, g_i8_d1, g_i9_d4);
}

// fn SGU 

