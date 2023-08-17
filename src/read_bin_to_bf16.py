def binary_to_bfloat16(binary_string):
    # 符号ビットを取得
    sgn = int(binary_string[0])

    # 指数部を取得し、10進数に変換
    exp = int(binary_string[1:9], 2)

    # 仮数部を取得し、10進数に変換
    fract = int(binary_string[9:], 2)
    # print("fract",fract)

    # bfloat16から浮動小数点数に変換
    if exp == 255:
        return float('inf') if fract == 0 else float('nan')
    if exp == 0:
        return 0.0

    # 符号部の計算
    sign = -1 if sgn else 1

    # 仮数部の計算
    fraction = (1 + fract / 128.0) #正しい
    # print("fraction", fraction)

    # 指数部の計算
    exponent = exp - 126

    # 浮動小数点数に変換
    float_value = sign * (fraction) * (2 ** exponent)

    return float_value


# テキストファイルからバイナリ列を読み込み
with open('w_value_bin.txt', 'r') as file:
    binary_values = file.read().strip().split(',')

# 各バイナリ列をbfloat16形式の浮動小数点数に変換
# float_values = [binary_to_bfloat16(binary) for binary in binary_values]
float_values = [binary_to_bfloat16(binary) for binary in ['1111011011001010','0100001001101101']]

# 結果を出力
for value in float_values:
    print(value)
    # continue