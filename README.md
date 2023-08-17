
## Requirement

` pip3 install tensorflow`

tensorflow version == 2.13.0 

## 行ったこと

1. w_value_bin.txt にランダムな16bit列を10000個書く。(gen_random_bin.pyで生成)
2. そのbit列をtensorflowでのbfloat16型に変換して、その型で加算。結果をまたbit列に直してから、w_tf_add_result.txtに書く。
3. w_value_bin.txt のbit列を次は自分の組んだモジュール(論理回路の処理を仮想的に表現したもの)で加算。
4. 3.の結果はそのままbit列なので、それをw_self_add_result.txtに書く。
5. 最後に2.,4.で作ったtxtが全て正しいか比較する。