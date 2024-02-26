fn psi_and(pa:f64,pb:f64)->f64 {
    let p1 = 9.0_f64 * 0.38_f64 / 4.0_f64;
    
    let pout:f64 =  0.38_f64 *( (pa).sqrt()+(pb).sqrt() - (1.0_f64/ 4.0_f64).sqrt() ) 
    .powf(2.0); // - (p1/ 4.0_f64).sqrt() vs - (1.0_f64/ 4.0_f64).sqrt() とどっちが正しい？... 1.0の方が(1,0), (0,0)のOUTPUT強度が一致するので正解．
    return pout;
}

fn psi_or(pa:f64,pb:f64)->f64 {
    
    let pout:f64 =  0.38_f64 * (
        ( (pa).sqrt()- 0.5_f64*(pb).sqrt() ).powf(2.0) 
        +
        (3.0_f64/4.0_f64 * pb) 
        ); 
    return pout;
}

fn dc_or(pa:f64,pb:f64)->f64 {
    
    let pout:f64 =  0.5_f64 * (
         pa + pb + 2.0_f64*(pa*pb).sqrt() 
        ); 
    return pout;
}


fn main() {
    // psi_and とpsi_orのカスケード接続は意味がない．理由はpsi_andとpsi_orの１とみなす情報の値が大きく異なるから．
    // psi_and(1,1)=0.855 ... "1" state. BUT psi_or(1,1)=0.38 ... "1" state. 

    let p_a:f64 = 1.0;
    let p_b:f64 = 1.0;
    let p_out = dc_or(p_a,p_b); //これらdc_or をpsi_andに変えればpsi_andの物理特性を見れる．
    println!("Pout at (1,1)= {}",p_out);
    
    let p_out2 = dc_or(1.0,0.0);
    println!("Pout at (1,0) = {}",p_out2);
    println!("BC contrast = {}",10.0*(p_out/p_out2).log10());
    
    let p_out3 = dc_or(1.0,0.38);
    println!("Pout on AND at (1,0.38) = {}",p_out3);
    let p_out4 = psi_and(0.57,0.38);
    println!("Pout on AND at (0.56,0.38) = {}",p_out4);
    
    for i in 0..2 {
        let p_a = (i as f64) as f64;
        for j in 0..11 {
            let p_b = (j as f64/10.0) as f64;
            let p_out = dc_or(p_a, p_b);
            println!("Pout at ({}, {}) = {}",p_a, p_b, p_out);
        }
    }
}