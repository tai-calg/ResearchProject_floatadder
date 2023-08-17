import random

# ランダムな16ビットのバイナリ列を生成する関数
def generate_random_binary():
    return ''.join(str(random.randint(0, 1)) for _ in range(16))

# 10,000個のランダムなバイナリ列を生成
binary_values = [generate_random_binary() for _ in range(10000)]

# これらのバイナリ列を','で区切ってテキストファイルに書き込み
with open('w_value_bin.txt', 'w') as file:
    file.write(','.join(binary_values))
