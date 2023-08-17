

# ================================================================================================= #

def bin_to_bf16(binary_string):
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



def bf16_to_bin(bfloat_val): #float -> str
    # 浮動小数点数が0の場合
    if bfloat_val == 0.0:
        return "0" * 16

    # 符号ビットの取得
    sgn = '1' if bfloat_val < 0 else '0'

    # 特殊なケースの処理（無限大とNaN）
    if bfloat_val == float('inf'):
        return sgn + '11111111' + '0' * 7
    elif bfloat_val == float('-inf'):
        return '1' + '11111111' + '0' * 7
    elif bfloat_val != bfloat_val:  # NaNをチェック
        return sgn + '11111111' + '1' * 7

    bfloat_val = abs(bfloat_val)

    # 仮数部と指数部を取得
    exp_val = 0
    if bfloat_val < 1.0:
        while bfloat_val < 1.0:
            bfloat_val *= 2
            exp_val -= 1
    else:
        while bfloat_val >= 2.0:
            bfloat_val /= 2
            exp_val += 1

    # 仮数部を2進数に変換
    fract = bfloat_val - 1.0  # 1.xxxxxの形式から整数部を取り除く
    fract_bin = ""
    for _ in range(7):  # bfloat16の仮数部は7ビット
        fract *= 2
        if fract >= 1:
            fract_bin += '1'
            fract -= 1.0
        else:
            fract_bin += '0'

    # 指数部を2進数に変換
    exp_val += 126
    exp_bin = format(exp_val, '08b')  # 8ビットの2進数に変換

    return sgn + exp_bin + fract_bin

# Test
# float_val:float = bin_to_bf16("0100000001000000")
# print(float_val)  # Expected output: 2.5
# binary_str = bf16_to_bin(float_val)
# print(binary_str)  # Expected output: 0100000001000000


