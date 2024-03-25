import matplotlib.pyplot as plt

# https://pythondatascience.plavox.info/matplotlib/%E8%89%B2%E3%81%AE%E5%90%8D%E5%89%8D


# psで計測
late_mzi = 0.4758
late_dc = 0.2334
late_ps = 0.009043
late_mrr = 0.3664
late_pd = 40.0
late_adc1 = 10.0
late_dac1 = 10.0
late_adc11 = 330.0
late_dac10 = 125.0
late_awg = 186.7
late_psi = 0.035
late_logicgate = 38.0

# psで計測
late_bf16a = 2.97*12/78 *1000.0 #ps

late_WMAU = late_dac10+late_mrr+late_awg+late_adc11
# late_WMAU_list = [late_dac10, late_mrr, late_awg, late_adc11]
late_WMAU_list = [late_adc11]
# late_WMAU_namelist = ['DAC_10', 'MRR', 'AWG', 'ADC_11']
late_WMAU_namelist = ['ADC_11']

late_OPA = late_logicgate+late_mrr+late_dc+late_psi*4. + late_adc1 + late_logicgate 
# late_OPA_list = [late_mrr, late_dc*4., late_psi*4., late_adc1, late_logicgate*2]
late_OPA_list = [late_mrr, late_dc*4., late_psi*4., late_logicgate*2]
# late_OPA_namelist = ['MRR', 'DC', 'PSI_AND', 'ADC_1', 'LogicGate']
late_OPA_namelist = ['MRR', 'DC', 'PSI_AND', 'LogicGate']

late_EPALU = 50.0 #ps





# 帯グラフの描画
fig, ax = plt.subplots()

# ~~~ with EPALU version ~~~ #

# # p_ebfa_fJperFPAOの描画
# ### 電気回路の割合を帯グラフで描画する

# ax.broken_barh([(0, late_bf16a)], (30, 9), facecolors='#ffaa00', label='10bitAdder')



# # p_WMAUの描画
# start = 0.0
# # colors = ['#0083ff','#ffff33','#ffaadd','#ff7070']
# colors = ['#ff7070']
# for i, latency in enumerate(late_WMAU_list):
#     ax.broken_barh([(start, latency)], (20, 9), facecolors=colors[i], label=late_WMAU_namelist[i])
#     start += latency


#     # ax.broken_barh([(0, power_WMAU[0])], (20, 9), facecolors='#0083ff', label=components_WMAU[0])
#     # ax.broken_barh([(power_WMAU[0], power_WMAU[1])], (20, 9), facecolors='#ffff33')
#     # ax.broken_barh([(sum(power_WMAU[:2]), power_WMAU[2])], (20, 9), facecolors='#ff7070', label=components_WMAU[2])

# # p_epalu の描画
# ax.broken_barh([(0, late_EPALU)], (10, 9), facecolors='#33bb54', label='EPALU')

# # p_OPAの描画
# start = 0
# # colors = ['#ffff33', 'y', 'm','#ff0000','#888888']
# colors = ['#ffff33', 'y', 'm','#888888']
# for i, latency in enumerate(late_OPA_list):
#     ax.broken_barh([(start, latency)], (0, 9), facecolors=colors[i], label=late_OPA_namelist[i])
#     start += latency

# # グラフの設定
# ax.set_ylim(0, 40)
# ax.set_xlim(0, max(sum(late_WMAU_list)+260, sum(late_OPA_list)+260, late_bf16a+260, late_EPALU+260 ))
# ax.set_xlabel('Latency [ps]')
# ax.set_yticks([5, 15,25, 35])
# ax.set_yticklabels(['L_EPALU','L_OPA', 'L_WMA', 'L_RTL10A' ])
# ax.grid(False)


# without EPALU version

# p_ebfa_fJperFPAOの描画
### 電気回路の割合を帯グラフで描画する

ax.broken_barh([(0, late_bf16a)], (20, 9), facecolors='#ffaa00', label='10bitAdder')



# p_WMAUの描画
start = 0.0
# colors = ['#0083ff','#ffff33','#ffaadd','#ff7070']
colors = ['#ff7070']
for i, latency in enumerate(late_WMAU_list):
    ax.broken_barh([(start, latency)], (10, 9), facecolors=colors[i], label=late_WMAU_namelist[i])
    start += latency


    # ax.broken_barh([(0, power_WMAU[0])], (20, 9), facecolors='#0083ff', label=components_WMAU[0])
    # ax.broken_barh([(power_WMAU[0], power_WMAU[1])], (20, 9), facecolors='#ffff33')
    # ax.broken_barh([(sum(power_WMAU[:2]), power_WMAU[2])], (20, 9), facecolors='#ff7070', label=components_WMAU[2])

# p_epalu の描画
# ax.broken_barh([(0, late_EPALU)], (10, 9), facecolors='#33bb54', label='EPALU')

# p_OPAの描画
start = 0
# colors = ['#ffff33', 'y', 'm','#ff0000','#888888']
colors = ['#ffff33', 'y', 'm','#888888']
for i, latency in enumerate(late_OPA_list):
    ax.broken_barh([(start, latency)], (0, 9), facecolors=colors[i], label=late_OPA_namelist[i])
    start += latency

# グラフの設定
ax.set_ylim(0, 30)
ax.set_xlim(0, max(sum(late_WMAU_list)+260, sum(late_OPA_list)+260, late_bf16a+260, late_EPALU+260 ))
ax.set_xlabel('Latency [ps]')
ax.set_yticks([5, 15,25])
ax.set_yticklabels(['L_OPA', 'L_WMA', 'L_RTL10A' ])
ax.grid(False)




# 凡例の追加
ax.legend()

# plt.savefig("optics-latency.png",dpi=300)
plt.savefig("optics-latency-wo-EPALU.png",dpi=300)
plt.show()