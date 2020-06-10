import matplotlib.pyplot as plt
import matplotlib
import os
import statistics

# plots:

# graph over max, average and smallest agent size
# graph over max, average and smallest maze size
# graph over max, average and smallest maze complexity

pre_avg_agent_sizes = []
pre_largest_agent_sizes = []
pre_smallest_agent_sizes = []

pre_avg_maze_sizes = []
pre_largest_maze_sizes = []
pre_smallest_maze_sizes = []

pre_avg_maze_complexities = []
pre_largest_maze_complexities = []
pre_smallest_maze_complexities = []

# experiment = "species_replacement"
# experiment = "varied_species"
experiment = "regular_speciated_mcc"

for root, dirs, files in os.walk(experiment, topdown=False):
    for name in files:
        avg_agent_size = []
        largest_agent_size = []
        smallest_agent_size = []

        avg_maze_size = []
        largest_maze_size = []
        smallest_maze_size = []

        avg_maze_complexity = []
        largest_maze_complexity = []
        smallest_maze_complexity = []

        for line in open(os.path.join(root, name), 'r', errors='ignore'):
            values = [float(s) for s in line.split()]

            avg_agent_size.append(values[9])
            largest_agent_size.append(values[10])
            smallest_agent_size.append(values[11])

            avg_maze_size.append(values[3])
            largest_maze_size.append(values[4])
            smallest_maze_size.append(values[5])

            avg_maze_complexity.append(values[6])
            largest_maze_complexity.append(values[7])
            smallest_maze_complexity.append(values[8])

        pre_avg_agent_sizes.append(avg_agent_size)
        pre_largest_agent_sizes.append(largest_agent_size)
        pre_smallest_agent_sizes.append(smallest_agent_size)

        pre_avg_maze_sizes.append(avg_maze_size)
        pre_largest_maze_sizes.append(largest_maze_size)
        pre_smallest_maze_sizes.append(smallest_maze_size)

        pre_avg_maze_complexities.append(avg_maze_complexity)
        pre_largest_maze_complexities.append(largest_maze_complexity)
        pre_smallest_maze_complexities.append(smallest_maze_complexity)

avg_agent_sizes = []
largest_agent_sizes = []
smallest_agent_sizes = []

avg_maze_sizes = []
largest_maze_sizes = []
smallest_maze_sizes = []

avg_maze_complexities = []
largest_maze_complexities = []
smallest_maze_complexities = []

for i in range(len(pre_avg_agent_sizes[0])):
    avg_agent_size = []
    largest_agent_size = []
    smallest_agent_size = []

    avg_maze_size = []
    largest_maze_size = []
    smallest_maze_size = []

    avg_maze_complexity = []
    largest_maze_complexity = []
    smallest_maze_complexity = []

    for j in range(len(pre_avg_agent_sizes)):
        avg_agent_size.append(pre_avg_agent_sizes[j][i])
        largest_agent_size.append(pre_largest_agent_sizes[j][i])
        smallest_agent_size.append(pre_smallest_agent_sizes[j][i])

        avg_maze_size.append(pre_avg_maze_sizes[j][i])
        largest_maze_size.append(pre_largest_maze_sizes[j][i])
        smallest_maze_size.append(pre_smallest_maze_sizes[j][i])

        avg_maze_complexity.append(pre_avg_maze_complexities[j][i])
        largest_maze_complexity.append(pre_largest_maze_complexities[j][i])
        smallest_maze_complexity.append(pre_smallest_maze_complexities[j][i])

    avg_agent_sizes.append(avg_agent_size)
    largest_agent_sizes.append(largest_agent_size)
    smallest_agent_sizes.append(smallest_agent_size)

    avg_maze_sizes.append(avg_maze_size)
    largest_maze_sizes.append(largest_maze_size)
    smallest_maze_sizes.append(smallest_maze_size)

    avg_maze_complexities.append(avg_maze_complexity)
    largest_maze_complexities.append(largest_maze_complexity)
    smallest_maze_complexities.append(smallest_maze_complexity)

avg_agent_sizes_sd = []
max_agent_sizes_sd = []
min_agent_sizes_sd = []

avg_maze_sizes_sd = []
max_maze_sizes_sd = []
min_maze_sizes_sd = []

avg_maze_complexity_sd = []
max_maze_complexity_sd = []
min_maze_complexity_sd = []

for i in range(len(avg_agent_sizes)):

    avg_agent_sizes_sd.append(statistics.stdev(avg_agent_sizes[i]))
    max_agent_sizes_sd.append(statistics.stdev(largest_agent_sizes[i]))
    min_agent_sizes_sd.append(statistics.stdev(smallest_agent_sizes[i]))

    avg_maze_sizes_sd.append(statistics.stdev(avg_maze_sizes[i]))
    max_maze_sizes_sd.append(statistics.stdev(largest_maze_sizes[i]))
    min_maze_sizes_sd.append(statistics.stdev(smallest_maze_sizes[i]))

    avg_maze_complexity_sd.append(statistics.stdev(avg_maze_complexities[i]))
    max_maze_complexity_sd.append(statistics.stdev(largest_maze_complexities[i]))
    min_maze_complexity_sd.append(statistics.stdev(smallest_maze_complexities[i]))

    avg_agent_sizes[i] = statistics.mean(avg_agent_sizes[i])
    largest_agent_sizes[i] = statistics.mean(largest_agent_sizes[i])
    smallest_agent_sizes[i] = statistics.mean(smallest_agent_sizes[i])

    avg_maze_sizes[i] = statistics.mean(avg_maze_sizes[i])
    largest_maze_sizes[i] = statistics.mean(largest_maze_sizes[i])
    smallest_maze_sizes[i] = statistics.mean(smallest_maze_sizes[i])

    avg_maze_complexities[i] = statistics.mean(avg_maze_complexities[i])
    largest_maze_complexities[i] = statistics.mean(largest_maze_complexities[i])
    smallest_maze_complexities[i] = statistics.mean(smallest_maze_complexities[i])


font = {'family': 'arial',
        'color': 'black',
        'weight': 'normal',
        'size': 14,
        }

plt.style.use('seaborn-whitegrid')


def save_plot(data, sd, y_label, file_name, y_max, y_min):
    x, y = [], []
    for i in range(len(data)):
        x.append(i)
        y.append(data[i])

    plt.plot(x, y)

    ma_sd, mi_sd = [], []
    for i in range(len(y)):
        ma_sd.append(y[i] + sd[i])
        mi_sd.append(y[i] - sd[i])

    plt.fill_between(range(len(y)), ma_sd, mi_sd, alpha=.1)

    plt.ylabel(y_label, fontsize=14, fontdict=font)
    plt.xlabel("Generations", fontsize=14, fontdict=font)

    plt.ylim(y_min, y_max)

    plt.savefig(file_name, bbox_inches="tight")
    plt.close()


def save_avg_max_min(avg, max, min, avg_sd, max_sd, min_sd, y_label, file_name):
    x, y = [], []
    for i in range(len(max)):
        x.append(i)
        y.append(max[i])

    plt.plot(x, y, label="max")

    ma_sd, mi_sd = [], []
    for i in range(len(max)):
        ma_sd.append(y[i] + max_sd[i])
        mi_sd.append(y[i] - max_sd[i])

    plt.fill_between(range(len(y)), ma_sd, mi_sd, alpha=.1)

    x, y = [], []
    for i in range(len(avg)):
        x.append(i)
        y.append(avg[i])

    plt.plot(x, y, label="avg")

    ma_sd, mi_sd = [], []
    for i in range(len(avg)):
        ma_sd.append(y[i] + avg_sd[i])
        mi_sd.append(y[i] - avg_sd[i])

    plt.fill_between(range(len(y)), ma_sd, mi_sd, alpha=.1)

    x, y = [], []
    for i in range(len(min)):
        x.append(i)
        y.append(min[i])

    plt.plot(x, y, label="min")

    ma_sd, mi_sd = [], []
    for i in range(len(min)):
        ma_sd.append(y[i] + min_sd[i])
        mi_sd.append(y[i] - min_sd[i])

    plt.fill_between(range(len(y)), ma_sd, mi_sd, alpha=.1)

    plt.legend(loc="upper left")

    plt.ylabel(y_label, fontsize=14, fontdict=font)
    plt.xlabel("Generations", fontsize=14, fontdict=font)

    plt.savefig(file_name, bbox_inches="tight")
    plt.close()


save_avg_max_min(avg_maze_sizes, largest_maze_sizes, smallest_maze_sizes, avg_maze_sizes_sd, max_maze_sizes_sd,
                 min_maze_sizes_sd, "Maze dimensions", "avg_max_min_maze_sizes.png")

save_avg_max_min(avg_agent_sizes, largest_agent_sizes, smallest_agent_sizes, avg_agent_sizes_sd, max_agent_sizes_sd,
                 min_agent_sizes_sd, "Agent genome connections",
                 "avg_max_min_agent_sizes.png")

save_avg_max_min(avg_maze_complexities, largest_maze_complexities, smallest_maze_complexities, avg_maze_complexity_sd,
                 max_maze_complexity_sd,
                 min_maze_complexity_sd, "Maze junctures",
                 "avg_max_min_maze_complexities.png")
