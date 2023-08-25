import numpy as np
import tensorflow as tf
import convert_type as ct

## read w_value_bin.txt as string list
with open('w_value_bin.txt', 'r') as file:
    bin_values = file.read().strip().split(',')
## any bin_values , ct.bin_to_bf16
w_np = np.array(list(map(lambda x: ct.bin_to_bf16(x), bin_values)))
# w_np = np.loadtxt('w_value_bin.txt', delimiter=',')




w_tf = tf.cast(w_np, tf.bfloat16) # 意味はないと思うが、正しさのために一応tfのbfloat16に変換
# print(type(w_tf), type(w_tf[0]))
# print("tf shape is",w_tf.shape)

##  [ w_tf[i] + w_tf[i+1] ) for i in range(0, len(w_tf), 2)]
# 偶数インデックスの要素をスライシングで取得
evens = w_tf[0::2] # [start:stop:step]
# 奇数インデックスの要素をスライシングで取得
odds = w_tf[1::2]
# 要素ごとの加算
result_tf = tf.add(evens, odds)
assert result_tf.shape == (5000,)





w_bin_np = result_tf.numpy()

## any w_bin_np , ct.bf16_to_bin() as string list
w_bin_list = list(map(lambda x: ct.bf16_to_bin(x), w_bin_np))
## write w_bin_np to w_b_value_bin.txt
with open('w_tf_add_result.txt', 'w') as f:
    f.write(','.join(w_bin_list))

print(type(w_bin_np[0]))

###
"""
from tensorflow.examples.tutorials.mnist import input_data
mnist = input_data.read_data_sets("/tmp/data/", one_hot=True)

from tensorflow.python import pywrap_tensorflow

bfloat16 = pywrap_tensorflow.TF_bfloat16_type()
import numpy as np
from b_bin import B_PARAM
from w_bin import W_PARAM
from float2bfloat import float2bfloat
from float2bfloat import bfloat2float

b = np.array(list(map(lambda x: bfloat2float(x), B_PARAM)))
w = np.array(list(map(lambda x: bfloat2float(x), W_PARAM)))

w = w.reshape(784, 10)

img=mnist.test.images[0]
print(img.shape, w.shape)

xw = img @ w
print(xw.shape)
rv = xw + b

print(rv)
"""
