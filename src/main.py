import random

def main():
    # 今回の例では、入力として指定した16ビットの値を使用していますが、必要に応じてランダムな16ビットの値を生成することもできます。
    # input1 = random.getrandbits(16)
    # input2 = random.getrandbits(16)
    input1 = 0b1011111100000101  # bfloat16
    input2 = 0b0011111100000011  # bfloat16
    print("### float adder ###")
    print("input1: {:0>16b}".format(input1))
    print("input2: {:0>16b}".format(input2))

    output = float_adder_run(input1, input2)
    print("output: {:0>16b}".format(output))

    print("### float adder by ieee ###")
    
    ieeef32_1 = ieee_to_f32(bfloat16_to_ieee(input1))
    ieeef32_2 = ieee_to_f32(bfloat16_to_ieee(input2))
    print("ieeef32_1", ieeef32_1)
    print("ieeef32_2", ieeef32_2)
    res_f32 = ieeef32_1 + ieeef32_2
    res_u32 = float_to_ieee_bin(f32_to_ieee(res_f32) )
    print("res_f32", res_f32)
    print("res_u32",(res_u32))
    assert output == res_u32 >> 16


def float_adder_run(input1:int, input2:int) -> int:
    sign_mask = 0b1000_0000_0000_0000
    in_sign1 = (input1 & sign_mask) != 0
    in_sign2 = (input2 & sign_mask) != 0

    exp_mask = 0b0111_1111_1000_0000
    in_exp1 = (input1 & exp_mask) >> 7
    in_exp2 = (input2 & exp_mask) >> 7

    fract_mask = 0b0000_0000_0111_1111
    in_fract1 = input1 & fract_mask
    in_fract2 = input2 & fract_mask

    sign_a, exp_a, fract_a = False, 0, 0
    sign_b, exp_b, fract_b = False, 0, 0

    if in_exp1 > in_exp2:
        sign_b, exp_b, fract_b = in_sign1, in_exp1, in_fract1
        sign_a, exp_a, fract_a = in_sign2, in_exp2, in_fract2
    elif in_exp1 == in_exp2:
        if in_fract1 > in_fract2:
            sign_b, exp_b, fract_b = in_sign1, in_exp1, in_fract1
            sign_a, exp_a, fract_a = in_sign2, in_exp2, in_fract2
        else:
            sign_a, exp_a, fract_a = in_sign1, in_exp1, in_fract1
            sign_b, exp_b, fract_b = in_sign2, in_exp2, in_fract2
    else:
        sign_a, exp_a, fract_a = in_sign1, in_exp1, in_fract1
        sign_b, exp_b, fract_b = in_sign2, in_exp2, in_fract2

    shift_val = min(exp_b - exp_a,8)
    fract_a |= 0b0000_0000_1000_0000
    fract_b |= 0b0000_0000_1000_0000

    fract_a = fract_a >> shift_val

    add_result = fract_b + fract_a
    sub_result = fract_b - fract_a

    selector = not (sign_a ^ sign_b)
    calc_result = add_result if selector else sub_result

    exp = exp_b
    fract = calc_result
    if selector: # add
        if (fract & 0b0000_0001_0000_0000) != 0:
            exp += 1
            fract = fract >> 1
    else: # sub
        while((fract & 0b0000_0000_1000_0000) == 0):
            exp -= 1
            fract = fract << 1

    exp_result = exp
    fract_result = fract & 0b0000_0000_0111_1111

    sign_and = sign_a & sign_b
    sign_xor = sign_a ^ sign_b
    is_minus = sign_and | sign_xor
    sign_result = is_minus & sign_b

    # bind sgn | exp | fract
    result = (int(sign_result) << 15) | (exp_result << 7) | fract_result

    return result


def u32_to_bool_array(n):
    return [(n >> i) & 1 == 1 for i in range(32)]

def bfloat16_to_ieee(bf16):
    sign = (bf16 & 0b1000000000000000)
    exp = (bf16 & 0b0111111110000000)
    fract = (bf16 & 0b0000000001111111)
    return (sign << 16) | (exp << 16) | (fract << 16)

def ieee_to_f32(ieee):
    return float.fromhex(hex(ieee))
import struct

def f32_to_ieee(f):
    return int.from_bytes(bytearray(struct.pack('f', f)), byteorder='big')

def float_to_ieee_bin(f):
    packed = struct.pack('!f', f)
    integer_representation = int.from_bytes(packed, byteorder='big')
    binary_representation = format(integer_representation, '032b')
    return binary_representation


if __name__ == "__main__":
    main()
