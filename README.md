
## Requirement

` pip3 install tensorflow`

tensorflow version == 2.13.0 

## 行ったこと

1. gen_random_bin.pyによって，w_value_bin.txt にランダムな16bit列を約10000個書く。
2. bfloat.pyによって，その約10000個の16bitをtensorflowでのbfloat16型の実数値に変換して，加算。結果をもう一度16bitに直してから、w_tf_add_result.txtに書く。
3. main.rsによって，w_value_bin.txt の16bitを次は自分の組んだモジュール(論理回路の処理を仮想的に表現したもの)で加算。
4. "3."の結果はそのまま16bitなので、それをw_self_add_result.txtに書く。
5. 最後に"2.","4."で作ったtxt(加算結果の約5000個の16bit)が全て正しいか比較する。


![float-adder-github drawio (1)](https://github.com/tai-calg/ResearchProject_floatadder/assets/62682789/3eae20c3-de9e-4929-9a7d-af6b243c23ac)



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
