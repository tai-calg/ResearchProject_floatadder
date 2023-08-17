import convert_type as ct
if __name__ == '__main__':
    import os
    script_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(script_dir, 'w_value_bin.txt')
        # filepath = トップディレクトリからの相対パス（どこから実行しても同じパスになるので便利）

    with open(file_path, 'r') as file:
        bin_values = file.read().strip().split(',')

    # 各バイナリ列をbfloat16形式の浮動小数点数に変換
    f_vals = [ct.bin_to_bf16(binary) for binary in bin_values]
    # f_vals = [bin_to_bf16(binary) for binary in ['1111011011001010','0100001001101101']]

# a+b
    sum_binary_strings = [ ct.bf16_to_bin(f_vals[i] + f_vals[i+1] ) for i in range(0, len(f_vals), 2)]
    assert len(sum_binary_strings) == 5000


    # Write the result to a file
    with open('add_results.txt', 'w') as f:
        f.write(','.join(sum_binary_strings))
    # 結果を出力
    # for value in f_vals:
    #     print(value)
        # continue
