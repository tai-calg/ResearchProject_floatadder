

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

w_value_bin_path = os.path.join(script_dir, 'w_value_bin.txt')
with open(w_value_bin_path,"r") as file:
    w_value_bin_strs = file.read().strip().split(',')


# compare #
count = 0
print("all count is",len(self_bin_strs))
for i in range(len(self_bin_strs)):
    if self_bin_strs[i] != tf_bin_strs[i]:
        count += 1
        print("------------\n not corrected !!",i, \
              "\n self is ", self_bin_strs[i][:1] , "_",self_bin_strs[i][1:9] , "_" , self_bin_strs[i][9:]  \
              ,"\n tf is   ", tf_bin_strs[i][:1] , "_",tf_bin_strs[i][1:9] , "_" , tf_bin_strs[i][9:] \
              , "\n  whose inputs are \n", w_value_bin_strs[2*i][:1], "_" , w_value_bin_strs[2*i][1:9] , "_" , w_value_bin_strs[2*i][9:] \
              , "and\n", w_value_bin_strs[2*i+1][:1] , "_" ,w_value_bin_strs[2*i+1][1:9] , "_" , w_value_bin_strs[2*i+1][9:]  \
              ,"\n----------\n"   )
        continue
    else:
        continue

print("different count:", count)