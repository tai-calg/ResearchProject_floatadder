import numpy as np
import tensorflow as tf

w_np = np.loadtxt('w_value.txt', delimiter=',')
b_np = np.loadtxt('b_value.txt', delimiter=',')

print(w_np)
print(type(w_np), type(w_np[0]))

w_b = tf.cast(w_np, tf.bfloat16)
b_b = tf.cast(b_np, tf.bfloat16)

print(type(w_b), type(w_b[0]))


w_b_np = w_b.numpy()
b_b_np = b_b.numpy()

print(type(w_b_np[0]))
print(w_b_np)

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
