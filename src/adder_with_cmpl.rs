use std::cmp;
// ビット列を分離(分割)したら，それぞれ別の変数として分けた方が可読性が良い．


pub fn adder_with_cmpl_run(input1:u32, input2:u32)->u32 {

    // === procedual 1 : swap ===
        let sign_mask = 0b1000_0000_0000_0000; // [15]
        let in_sign1:bool = (input1 & sign_mask) != 0;
        let in_sign2:bool = (input2 & sign_mask) != 0;
    
        let exp_mask = 0b0111_1111_1000_0000; // [14:7]
        let in_exp1 = (input1 & exp_mask) >> 7; // 7bit右シフトして範囲を0~255にする
        let in_exp2 = (input2 & exp_mask) >> 7; 
        
        let fract_mask = 0b0000_0000_0111_1111; // [6:0]
        
        let in_fract1 = input1 & fract_mask;
        let in_fract2 = input2 & fract_mask;
    
        let sign_a; // false: +, true: -
        let exp_a;
        let mut fract_a;
    
        let sign_b;
        let exp_b;
        let mut fract_b;
    
        // MUST b > a
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
        fract_a |= 0b0000_0000_1000_0000; //hidden bitを結合, 8bit
        fract_b |= 0b0000_0000_1000_0000; //hidden bitを結合, 8bit
    
    // === toCmpl. ===
    let xor = sign_a ^ sign_b;
    if xor { // case sub
        // 10bitであるfract_a をbit反転 , +1 
        fract_a = ( (!fract_a & 0xf_f_f_f_f) + 1 ) ; //GRS 10bit+shift max scaler 10bit
            // !shifted_fr_aはu32で32bitすべてが反転するのでダメ
        assert!( (!(0b01100)+1) == (0b01100 * -1) );

        /*    
        println!("{}", 0b01100);
        println!("{}", !(0b01100)+1);
        assert!( (!(0b01100)+1) == (0b01100 * -1) );
        println!("succeess"); 
        */
        
    }else{ // case add
        //nothing to do
    }



        
        // === procedual 2 : shift ===

        
        
        //ゼロ例外のときvalue＝0にする．(inf例外は後で対応してる)（非正規数はそもそも入力されない前提）
                if exp_a == 0b000_0000 {
                    fract_a = 0;
                }
                if exp_b == 0b000_0000 {
                    fract_b = 0;
                }
    
    
        let mut shift_val = cmp::min(exp_b - exp_a,10);  
        // これによってfract_aを分離したい
        // (fract_a >> n) into shifted_fr_a
        let for_round_b4grs = fract_a & ((1<<shift_val) - 1);
        assert!( ((1<<4) -1) == 0b1111); 
        let mut shifted_fr_a = fract_a >> shift_val; 



    
    
    // === procedual 3 : add , sub ===
        let addsub_result = (shifted_fr_a + fract_b) & 0b01_1111_1111; //9bit, because 10bit as sign bit must be 0. 
        assert!(( (shifted_fr_a + fract_b) & 0b10_0000_0000 ) == 0); // 10th bit is 0
            
    
    // === procedual 4 : normalize ===
        let mut guard = false;

        /*
        Addを選択した時…  if (    fr[8](...最上位ビットである9番目) == 1    ){exp++; fr >>1;}else{} fractはinput[6:0]を出力
        Subを選択した時…  while (   fr[7](...切り捨てる予定の整数部) == 0    ) { exp--; fr << 1;} while後にinput[6:0]を出力
         */
        let mut exp = exp_b;
    
        let floor_mask:u32 = 1 << 7; // 1000_0000
        let fract_mask:u32 = (1 << 7) - 1; // 0111_1111
        let mut result_bind = (addsub_result<< shift_val ) | for_round_b4grs; //9+shift_val bit
    
        if !xor { // add
            if (addsub_result & (floor_mask<<1)) != 0 { // (桁あがりしてる時) 
                exp += 1;

                // result_bind = result_bind >> 1;// shiftval=0,1の時だけここで情報ロスが起こる．
                //### ならば， >>1するのではなく，マスクを左に1ずらす！ ###
                shift_val += 1;
    
            }
        }else { // sub

            while (result_bind & (floor_mask<< shift_val)) == 0 { 
                exp -= 1;
                if exp == 0 { return 0b0_00000000_0000000;} //exp<7で発生する可能性あり．
                result_bind = result_bind << 1;
            }
        }

        // 分解
        let grs = result_bind & ((1 << shift_val) - 1); //shift_val bit . ただし人間が下からの0 padding 含めて10bitとみなす．
        let mut fract = (result_bind >> shift_val) & 0b0_0111_1111; //9bit -> 7bit

        
        
        
        // === procedual 5  : round ===
        let ulp =  ( fract & 0b1 )==1  ;
        let mut round = false;
        let mut sticky_mask = 0;

        if shift_val >= 1 {
            guard = (grs >> (shift_val-1)) & 0b1 == 1;
        }
        if shift_val >= 2 {
            round = (grs >> (shift_val-2)) & 0b1 == 1;
            sticky_mask = (1 << (shift_val-2)) - 1;
        }
        let sticky = (grs & sticky_mask) >=1 ; //k bit OR

        let fr_all1 = fract == 0b0111_1111;
    
    
        if guard & (sticky | round | ulp) {
            if fr_all1 {
                fract = 0;
                exp += 1;
            }else{
                fract += 1;
            }
        }
    
    
    // === procedual 6 : 例外処理 ===
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
    let fract_result = fract & 0b0000_0000_0111_1111; 
    let sign_result = sign_b;
    
    //inf例外を考慮
    if exp_result == 0b11111111 {
        // +inf or -inf
        return (sign_result as u32) << 15 | 0b0_11111111_0000000;
    }
    
    // === procedual 7 : binding bits process  ===
    
        //bind sgn | exp | fract
        let result = (sign_result as u32) << 15 | (exp_result << 7) | fract_result;
    
        return result; 
    
    }
    