
## Requirement

` pip3 install tensorflow`

tensorflow version == 2.13.0 

## 行ったこと

1. gen_random_bin.pyによって，w_value_bin.txt にランダムな16bit列を約10000個書く。
2. bfloat.pyによって，その約10000個の16bitをtensorflowでのbfloat16型の実数値に変換して，加算。結果をもう一度16bitに直してから、w_tf_add_result.txtに書く。
3. main.rsによって，w_value_bin.txt の約10000個の16bitを次は自分の組んだモジュール(論理回路の処理を仮想的に表現したもの)で加算。結果はそのまま16bitなので、それをw_self_add_result.txtに書く。
4. 最後にcompare.pyによって，"2.","3."で作ったtxt(加算結果の約50000個の16bit)が全て一致するか比較する。(自分で実装したBfloat16 Adderが正しいかどうかを確認する)


![float-adder-github drawio](/img/float-adder-github.png)

## 使い方

### 全体の流れ
```
$ python gen_random_bin.py

$ python bfloat.py

$ cargo run  --  --adder Default

$ python compare.py 
```

#### 使用する計算方法を選択する
`cargo run  --  --adder [AdderName]`でBfloat16加算器の内部処理回路の変更ができます．

選択肢は以下の通りです．

`cargo run  --  --adder Default` ... 卒業論文，付録部(図7.1)に記載された補数表現を使用しないBfloat16加算器

`cargo run  --  --adder ByCmpl`   ...  卒業論文，第二章に記した補数表現を使用するBfloat16加算器

`cargo run  --  --adder TenAdderLG` ... 10bit Prefix Adder を論理ゲートで処理したBfloat16加算器

`cargo run  --  --adder TenAdderOPA` ... 10bit Prefix Adder を光デバイスのAND,ORゲートで処理したBfloat16加算器(本研究の提案)

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
2. `0_00000000_1010110`のような値は正規化数であるため，入力されます．これを全て0にする例外処理をコードの中に入れています．
3. +-infは入力されます．従って，Add/Subして正規化，丸めをした最終の計算結果の16bitを見て，計算結果が+-infかどうかを処理しています．
