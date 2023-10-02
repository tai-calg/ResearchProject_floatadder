import random

# ランダムな16ビットのバイナリ列を生成する関数
def generate_random_binary():
    """
    ランダムな16ビットのバイナリ列を生成します。
    
    Returns:
        str: 生成された16ビットのバイナリ列
    """
    return ''.join(str(random.randint(0, 1)) for _ in range(16))

# バイナリ列が条件に合致するかどうかをチェックする関数
def check_condition(binary_str):
    """
    与えられたバイナリ列の2ビット目から9ビット目が "11111111" であるかどうかをチェックします。
    
    Args:
        binary_str (str): チェックするバイナリ列
    
    Returns:
        bool: バイナリ列の2ビット目から9ビット目が "11111111" の場合は False、それ以外は True
    """
    return binary_str[1:9] != '11111111'

# 10,0000個のランダムなバイナリ列を生成し、条件に合致するものだけを保存
binary_values = [generate_random_binary() for _ in range(100000)]
# if binary_values.count % 2 == 1: # 奇数の場合は最後の要素を削除
if len(binary_values) % 2 == 1:
    binary_values.pop()
    
filtered_values = filter(check_condition, binary_values)

# これらのバイナリ列を','で区切ってテキストファイルに書き込み
with open('src/w_value_bin.txt', 'w') as file:
    file.write(','.join(filtered_values))
