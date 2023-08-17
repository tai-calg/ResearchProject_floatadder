# https://detail.chiebukuro.yahoo.co.jp/qa/question_detail/q1148638731

def binary_to_decimal(binary):
    binary = int(str(binary), 2)
    return binary

def decimal_to_binary(n, length):
    return format(n, '0{}b'.format(length))

def ieee754_to_float(b_str):
    sign = (-1) ** int(b_str[0])
    exponent = binary_to_decimal(b_str[1:9]) - 127
    mantissa = "1" + b_str[9:] # 仮数部
    print("変換前",mantissa)
    mantissa = sum([int(bit) * 2 ** (-i) for i, bit in enumerate(mantissa)]) # 仮数部を10進数に変換
    print("変換後",mantissa)
    return sign * mantissa * 2 ** exponent

def float_to_ieee754(value):
    sign_bit = '0' if value > 0 else '1'
    value = abs(value)
    exponent = 0
    while value < 1:
        value *= 2
        exponent -= 1
    while value >= 2:
        value /= 2
        exponent += 1
    mantissa = ''
    value -= 1  # remove the leading 1
    while len(mantissa) < 23 and value != 0:
        value *= 2
        if value >= 1:
            mantissa += '1'
            value -= 1
        else:
            mantissa += '0'
    exponent += 127
    return sign_bit + decimal_to_binary(exponent, 8) + mantissa.ljust(23, '0')

# Binary representations
A = "0100000010110010000000000000000"
B = "0100000111111001000000000000000"

# Convert to floating point
A_float = ieee754_to_float(A)
B_float = ieee754_to_float(B)

# Perform addition
result_float = A_float + B_float

# Convert back to binary
result_binary = float_to_ieee754(result_float)

print(result_binary)
