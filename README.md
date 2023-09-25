
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
    ↓                           ↓                 |
w_value_bin.txt(約10000個) -> w_tf_add_result.txt(約5000個)        ↓
    L--------------------------->      w_self_add_result.txt(約5000個) 
```

## 課題点

まだmain.rsが238個間違った計算をしている。
しかしそれはまだmain.rsがRound と 例外処理を入れてないからだと思いたい。

tfがulp1足りない場合(不要なところでselfが丸めで繰り上げしてる。)もあれば、selfがulp1足りない場合もあってよくわからん

add/subをした後に先にNorm→例外検知→Roundの順。
IF: Roundのせいで正規形でなくなったら、もう一度Normに戻す。

```
val    s_exponent_signcnd
+inf = 0_11111111_0000000
-inf = 1_11111111_0000000

val    s_exponent_signcnd
+NaN = 0_11111111_{not all 0}
-NaN = 1_11111111_{not all 0}

0    = 0 00000000 0000000 = 0
0    = 1 00000000 0000000 = −0


```
- tfは exp= 11111111 の時, fractは{all 0}(inf) or {all 1}(+/- Nan)しか出さない。

- 8/29
例外処理を実装。
現時点で残ってるミス
1. selfの方がulp 1大きい (why???)
2. tfがinfになってるのにselfはexp=1111_1110でオーバーフローしてない (infであることをinfでないとして出力するのは問題)
3. selfの方がulp 1小さい (Roundを実装してないからと予想してる)
4. self=-Nan ,but tf=+Nan (!?)

