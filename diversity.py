import matplotlib.pyplot as plt
import os
import statistics

regular_speciated_mcc_diversity = []
varied_species_diversity = []
species_replacement_diversity = []

for root, dirs, files in os.walk("regular_speciated_mcc_diversity", topdown=False):
    for name in files:
        for line in open(os.path.join(root, name), 'r'):
            values = [float(s) for s in line.split()]

            regular_speciated_mcc_diversity.append(values[0])

avg_regular_speciated_mcc_diversity = statistics.mean(regular_speciated_mcc_diversity)

for root, dirs, files in os.walk("varied_species_diversity", topdown=False):
    for name in files:
        for line in open(os.path.join(root, name), 'r'):
            values = [float(s) for s in line.split()]

            varied_species_diversity.append(values[0])

avg_varied_species_diversity = statistics.mean(varied_species_diversity)

for root, dirs, files in os.walk("species_replacement_diversity", topdown=False):
    for name in files:
        for line in open(os.path.join(root, name), 'r'):
            values = [float(s) for s in line.split()]

            species_replacement_diversity.append(values[0])

avg_species_replacement_diversity = statistics.mean(species_replacement_diversity)


avg_regular_speciated_mcc_diversity_sd = statistics.stdev(regular_speciated_mcc_diversity)
avg_varied_species_diversity_sd = statistics.stdev(varied_species_diversity)
avg_species_replacement_diversity_sd = statistics.stdev(species_replacement_diversity)


plt.style.use('seaborn-whitegrid')


fig = plt.figure()
ax = fig.add_subplot(111)

variants = ["Regular speciated MCC", "Varied size speciation", "Species replacement"]
values = [avg_regular_speciated_mcc_diversity, avg_varied_species_diversity, avg_species_replacement_diversity]
error = [avg_regular_speciated_mcc_diversity_sd, avg_varied_species_diversity_sd, avg_species_replacement_diversity_sd]

bars = ax.bar(variants, values, yerr=error, align='edge', ecolor='black', capsize=10)

bars[2].set_color('g')
bars[0].set_color('orange')

for bar in bars:
  ax.text(
      bar.get_x() + bar.get_width() / 2,
      bar.get_height() + 0.3,
      round(bar.get_height(), 3),
      horizontalalignment='right',
      weight='bold'
  )

ax.set_ylabel('Diversity score', labelpad=15, color='#333333')
ax.set_title('Diversity', pad=15, color='#333333',
             weight='bold')

plt.savefig("diversity.png", bbox_inches="tight")
plt.show()
