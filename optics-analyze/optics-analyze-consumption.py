import matplotlib.pyplot as plt

# https://pythondatascience.plavox.info/matplotlib/%E8%89%B2%E3%81%AE%E5%90%8D%E5%89%8D


"""
以下のPythonのコードのp_WMAUとp_OPNAの内訳を色で分けて，帯グラフで描いてください.
p_WMAUを１行で描いてください.
各色が何かをわかるためにラベルを付けてください.


Name , Power [fJ/回], Latency[s]
    MZI, 32.4, 4.758 e-13
    Directional Coupler, 16.2, 2.334 e-13
    Phase Shifter, 0, 9.043 e-15
    MRR, 24, 3.664 e-13
    PD, 1.2, 4.0 e-11
    ADC_1, 270.0, 1.0 e-11
    DAC_1, 153.0, 1.0 e-11
    ADC_11, 1059, 0.33 e-9
    DAC_10, 350.1, 0.125 e-9
    Multiplexer(AWG), 0, 1.867 e-10
    """
    
"""
p_pd	1.2
p_psi	1.7
p_ps	0
p_dc	0
p_logic	2.112998933
p_adc1	4.00E+00
p_adc11	4.10E+03
p_dac10	4.36E+00
p_awg	0
p_mrr	7
"""

# [fJ/bit]
p_mzi = 32.4
p_dc = 0.0
p_ps = 0.0
p_mrr = 7.0
p_pd = 1.2
p_adc1 = 4.0
# p_dac1 = .0
p_adc11 = 4100.0
p_dac10 = 4.36
p_psi = 1.7
p_lg = 1.79 / 336.7 / 2516* 1000000 # 2.113 fJ/bit



p_WMAU:float = p_dac10 *2 + p_mrr*2 + p_adc11 
p_OPNA:float = p_adc1 *10 +p_mrr*10 + p_dc*17 +p_pd*10+ p_psi*34 + p_lg*30

import numpy as np

# p_WMAとp_OPAの内訳
components_WMAU = ['DAC_10', 'MRR', 'ADC_11']
power_WMAU = [p_dac10 * 2, p_mrr * 2, p_adc11]

components_OPNA = [ 'MRR', 'DC','PD', 'PSI_AND','ADC_1', 'LogicGate']
power_OPNA = [ p_mrr * 10, p_dc * 17,p_pd*10, p_psi * 34, p_adc1 * 10, p_lg*30 ]

# 帯グラフの描画
fig, ax = plt.subplots()

# ~~~ with EPALU version ~~~ #

# # # p_ebfa_fJperFPAOの描画
# ## 電気回路の割合を帯グラフで描画する
# p_ebfa = 1.79 #mW 
# p_ebfa_fJperFPAO = p_ebfa / 336.7 *1000.0 *1000.0 #fJ/FPAO
# p_e10ad_fJperFPAO = p_ebfa_fJperFPAO * (345/(905+96+354+138+38+345+609+31))
# p_ebfa_else_fJperFPAO = p_ebfa_fJperFPAO - p_e10ad_fJperFPAO
# # ax.broken_barh([(0, p_ebfa_else_fJperFPAO)], (11, 10), facecolors='#00ff00', label='else')
# ax.broken_barh([(0, p_e10ad_fJperFPAO)], (30, 9), facecolors='#ffaa00', label='10bitAdder')



# # p_WMAUの描画
# ax.broken_barh([(0, power_WMAU[0])], (20, 9), facecolors='#0083ff', label=components_WMAU[0])
# ax.broken_barh([(power_WMAU[0], power_WMAU[1])], (20, 9), facecolors='#ffff33')
# ax.broken_barh([(sum(power_WMAU[:2]), power_WMAU[2])], (20, 9), facecolors='#ff7070', label=components_WMAU[2])

# # p_epalu の描画
# p_epalu = 1.0445 * 1000.0 #fJ/FPAO
# ax.broken_barh([(0, p_epalu)], (0, 9), facecolors='#33bb54', label='EPALU')

# # p_OPNAの描画
# start :float = 0
# colors = ['#ffff33', 'y','#000000', 'm','#ff0000','#888888']
# for i, power in enumerate(power_OPNA):
#     ax.broken_barh([(start, power)], (10, 9), facecolors=colors[i], label=components_OPNA[i])
#     start += power

# # グラフの設定
# ax.set_ylim(0, 40)
# ax.set_xlim(0, max(sum(power_WMAU)+1000, sum(power_OPNA)+1000, p_e10ad_fJperFPAO+1000 ))
# ax.set_xlabel('Energy [fJ/FPAO]')
# ax.set_yticks([5, 15, 25, 35])
# ax.set_yticklabels(['E_EPALU','E_OPA', 'E_WMA', 'E_RTL10A' ])
# ax.grid(False)




# ~~~ without EPALU version ~~~ #

## p_ebfa_fJperFPAOの描画
## 電気回路の割合を帯グラフで描画する
p_ebfa = 1.79 #mW 
p_ebfa_fJperFPAO = p_ebfa / 336.7 *1000.0 *1000.0 #fJ/FPAO
p_e10ad_fJperFPAO = p_ebfa_fJperFPAO * (345/(905+96+354+138+38+345+609+31))
p_ebfa_else_fJperFPAO = p_ebfa_fJperFPAO - p_e10ad_fJperFPAO
# ax.broken_barh([(0, p_ebfa_else_fJperFPAO)], (11, 10), facecolors='#00ff00', label='else')
ax.broken_barh([(0, p_e10ad_fJperFPAO)], (20, 9), facecolors='#ffaa00', label='10bitAdder')



# p_WMAUの描画
ax.broken_barh([(0, power_WMAU[0])], (10, 9), facecolors='#0083ff', label=components_WMAU[0])
ax.broken_barh([(power_WMAU[0], power_WMAU[1])], (10, 9), facecolors='#ffff33')
ax.broken_barh([(sum(power_WMAU[:2]), power_WMAU[2])], (10, 9), facecolors='#ff7070', label=components_WMAU[2])


# p_OPAの描画
start = 0.0
colors = ['#ffff33', 'y','#000000', 'm','#ff0000','#888888']

for i, power in enumerate(power_OPNA):
    ax.broken_barh([(start, power)], (0, 9), facecolors=colors[i], label=components_OPNA[i])
    start += power
    
# グラフの設定 without_EPALU
ax.set_ylim(0, 30)
ax.set_xlim(0, max(sum(power_WMAU)+300, sum(power_OPNA)+300, p_e10ad_fJperFPAO+300 ))
ax.set_xlabel('Energy [fJ]')
ax.set_yticks([5, 15,25])
ax.set_yticklabels(['E_OPA', 'E_WMAU', 'E_RTL10A' ])
ax.grid(False)

# ~~~ end of without EPALU version ~~~ #




# 凡例の追加
ax.legend()

plt.savefig("optics-power.png",dpi=300)
# plt.savefig("optics-analyze_without_EPALU.png",dpi=300)
plt.show() # これは最後に書かないとsavefigが真っ白になる．



print("P_bf16adder is" , p_ebfa_fJperFPAO)
print("P_10bitadder is" , p_e10ad_fJperFPAO)
print("P_WMAU is" , p_WMAU)
print("P_OPNA is" , p_OPNA)


"""
`broken_barh`関数の引数`(20, 9)`と`(30, 9)`は，それぞれ帯グラフの位置と高さを指定しています．

- `(20, 9)`の場合，`20`はy軸上の帯の開始位置を示し，`9`は帯の高さを示します．したがって，この帯はy軸の20から始まり，高さが9の位置まで続きます．
- 同様に，`(30, 9)`の場合，帯はy軸の30から始まり，高さが9の位置まで続きます．

これらの値を調整することで，帯グラフの各部分がどの位置に表示され，どの程度の高さを持つかを制御できます．
この場合，`P_bf16a`と`P_EPALU`の帯が重ならないように，それぞれ異なる開始位置を持つように設定されています．
"""