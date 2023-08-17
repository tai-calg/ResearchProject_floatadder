

# read 

import os
script_dir = os.path.dirname(os.path.abspath(__file__))
self_module_path = os.path.join(script_dir, 'w_self_add_result.txt')
    # filepath = トップディレクトリからの相対パス（どこから実行しても同じパスになるので便利）

with open(self_module_path, 'r') as file:
    self_bin_strs = file.read().strip().split(',')
    
script_dir = os.path.dirname(os.path.abspath(__file__))
tf_path = os.path.join(script_dir, 'w_tf_add_result.txt')
    # filepath = トップディレクトリからの相対パス（どこから実行しても同じパスになるので便利）

with open(tf_path, 'r') as file:
    tf_bin_strs = file.read().strip().split(',')

for i in range(len(self_bin_strs)):
    if self_bin_strs[i] != tf_bin_strs[i]:
        print("not corrected !!",i, self_bin_strs[i], tf_bin_strs[i])
        continue
    else:
        continue
    