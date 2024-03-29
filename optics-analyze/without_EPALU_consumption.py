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

p_mzi = 32.4
p_dc = 16.2
p_ps = 0.0
p_mrr = 24.0
p_pd = 1.2
p_adc1 = 270.0
p_dac1 = 153.0
p_adc11 = 1059.0
p_dac10 = 350.1
p_psi = 1.7
p_lg = 5*1/2516 *1000.0 #fJ/once



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

# p_ebfa_fJperFPAOの描画
### 電気回路の割合を帯グラフで描画する
p_ebfa = 1.79 #mW 
p_ebfa_fJperFPAO = p_ebfa / 336.7 *1000.0 *1000.0 #fJ/FPAO
p_e10ad_fJperFPAO = p_ebfa_fJperFPAO * (345/(905+96+354+138+38+345+609+31))
p_ebfa_else_fJperFPAO = p_ebfa_fJperFPAO - p_e10ad_fJperFPAO
# ax.broken_barh([(0, p_ebfa_else_fJperFPAO)], (11, 10), facecolors='#00ff00', label='else')
ax.broken_barh([(0, p_e10ad_fJperFPAO)], (20, 9), facecolors='#00cc00', label='10bitAdder')



# p_WMAUの描画
ax.broken_barh([(0, power_WMAU[0])], (10, 9), facecolors='#0083ff', label=components_WMAU[0])
ax.broken_barh([(power_WMAU[0], power_WMAU[1])], (10, 9), facecolors='#ffff33')
ax.broken_barh([(sum(power_WMAU[:2]), power_WMAU[2])], (10, 9), facecolors='#ff7070', label=components_WMAU[2])

# # p_epalu の描画
# p_epalu = 1.0445 * 1000.0 #fJ/FPAO
# ax.broken_barh([(0, p_epalu)], (0, 9), facecolors='#33bb54', label='EPALU')

# p_OPNAの描画
start = 0
colors = ['#ffff33', 'y','#000000', '#bb9900','#ff0000','#888888']
for i, power in enumerate(power_OPNA):
    ax.broken_barh([(start, power)], (0, 9), facecolors=colors[i], label=components_OPNA[i])
    start += power

# グラフの設定
ax.set_ylim(0, 30)
ax.set_xlim(0, max(sum(power_WMAU)+1000, sum(power_OPNA)+1000, p_e10ad_fJperFPAO+1000 ))
ax.set_xlabel('Energy [fJ]')
ax.set_yticks([5, 15, 25])
ax.set_yticklabels(['E_OPA', 'E_WMA', 'E_RTL10A' ])
ax.grid(False)

print(p_WMAU, p_e10ad_fJperFPAO)

# 凡例の追加
ax.legend()

plt.savefig("optics-power.png",dpi=300)
# plt.savefig("optics-analyze_without_EPALU.png",dpi=300)
plt.show() # これは最後に書かないとsavefigが真っ白になる．



print("P_bf16adder is" , p_ebfa_fJperFPAO)
print("P_10bitadder is" , p_e10ad_fJperFPAO)
print("P_WMAU is" , p_WMAU)
print("P_OPNA is" , p_OPNA)
