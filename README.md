
## Requirement

` pip3 install tensorflow`

tensorflow version == 2.13.0 

## 行ったこと

1. w_value_bin.txt にランダムな16bit列を10000個書く。(gen_random_bin.pyで生成)
2. そのbit列をtensorflowでのbfloat16型に変換して、その型で加算。結果をまたbit列に直してから、w_tf_add_result.txtに書く。
3. w_value_bin.txt のbit列を次は自分の組んだモジュール(論理回路の処理を仮想的に表現したもの)で加算。
4. 3.の結果はそのままbit列なので、それをw_self_add_result.txtに書く。
5. 最後に2.,4.で作ったtxtが全て正しいか比較する。

```
gen_random_bin.py  ==>     bfloat.py     ==>    main.rs     ==>  compare_txt.py 
    ↓                           ↓               　　Lーーーーーーーー|
w_value_bin.txt(約10000個) -> w_tf_add_result.txt(約5000個)        ↓
    L--------------------------->      w_self_add_result.txt(約5000個) 
```

## 注意点

### コードの読み方

Rustではbit演算を　AND(&)と shiftを行うことで処理してます．

つまりVerilogでvalue[12:3]のように切り取る操作は，Rustでは”ANDマスクによって切り取りたいBitを切り取ったのちにShift(>>)でスライドさせる”ことで実現しています．

例：
```
    let exp_mask = 0b0111_1111_1000_0000; // [14:7]
    let in_exp1 = (input1 & exp_mask) >> 7; 
```


---
### 例外
例外の値については以下のようになっています．

```
val    sgn_exponent_fraction
+inf = 0_11111111_0000000
-inf = 1_11111111_0000000

val    sgn_exponent_fraction
+NaN = 0_11111111_{not all 0}
-NaN = 1_11111111_{not all 0}

0    = 0 00000000 0000000 = 0
0    = 1 00000000 0000000 = 0
0    = {any} 00000000 {any} = 0

```

### 入力される値について

1. Nanは入力されない仕様にしています．
2. `0_00000000_1010110`のような値は正規化数です．故に入力されるので，これを全て0にする例外処理をコードの中に入れています．
3. +-infは入力されます．従って，Add/Subして正規化，丸めをした最終の計算結果の16bitを見て，計算結果が+-infかどうかを処理しています．
