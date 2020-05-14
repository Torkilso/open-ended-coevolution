import matplotlib.pyplot as plt
import matplotlib
import os
import statistics

# plots:

# all graphs use generation as x value

# graph over average agent amount
# graph over max, average and smallest agent size

# graph over average maze amount
# graph over max, average and smallest maze size
# graph over max, average and smallest maze complexity

# graph over overall average maze size increase
# graph over overall average maze complexity size increase


pre_agent_amounts = []
pre_avg_agent_sizes = []
pre_largest_agent_sizes = []
pre_smallest_agent_sizes = []

pre_overall_agent_size_increase = []

pre_maze_amounts = []
pre_avg_maze_sizes = []
pre_largest_maze_sizes = []
pre_smallest_maze_sizes = []
pre_avg_maze_complexities = []
pre_largest_maze_complexities = []
pre_smallest_maze_complexities = []

pre_overall_maze_size_increase = []
pre_overall_maze_complexity_increase = []

for root, dirs, files in os.walk("regular_speciated_mcc", topdown=False):
    for name in files:
        agent_amount = []

        avg_agent_size = []
        largest_agent_size = []
        smallest_agent_size = []

        overall_agent_size_increase = []

        maze_amount = []

        avg_maze_size = []
        largest_maze_size = []
        smallest_maze_size = []
        avg_maze_complexity = []
        largest_maze_complexity = []
        smallest_maze_complexity = []

        overall_maze_size_increase = []
        overall_maze_complexity_increase = []

        for line in open(os.path.join(root, name), 'r'):
            values = [float(s) for s in line.split()]

            agent_amount.append(values[1])

            avg_agent_size.append(values[9])
            largest_agent_size.append(values[10])
            smallest_agent_size.append(values[11])

            overall_agent_size_increase.append(values[15])

            maze_amount.append(values[2])

            avg_maze_size.append(values[3])
            largest_maze_size.append(values[4])
            smallest_maze_size.append(values[5])
            avg_maze_complexity.append(values[6])
            largest_maze_complexity.append(values[7])
            smallest_maze_complexity.append(values[8])

            overall_maze_size_increase.append(values[16])
            overall_maze_complexity_increase.append(values[17])

        pre_agent_amounts.append(agent_amount)

        pre_avg_agent_sizes.append(avg_agent_size)
        pre_largest_agent_sizes.append(largest_agent_size)
        pre_smallest_agent_sizes.append(smallest_agent_size)

        pre_overall_agent_size_increase.append(overall_agent_size_increase)

        pre_maze_amounts.append(maze_amount)
        pre_avg_maze_sizes.append(avg_maze_size)
        pre_largest_maze_sizes.append(largest_maze_size)
        pre_smallest_maze_sizes.append(smallest_maze_size)
        pre_avg_maze_complexities.append(avg_maze_complexity)
        pre_largest_maze_complexities.append(largest_maze_complexity)
        pre_smallest_maze_complexities.append(smallest_maze_complexity)

        pre_overall_maze_size_increase.append(overall_maze_size_increase)
        pre_overall_maze_complexity_increase.append(overall_maze_complexity_increase)

agent_amounts = []

avg_agent_sizes = []
largest_agent_sizes = []
smallest_agent_sizes = []

overall_agent_size_increases = []

maze_amounts = []

avg_maze_sizes = []
largest_maze_sizes = []
smallest_maze_sizes = []
avg_maze_complexities = []
largest_maze_complexities = []
smallest_maze_complexities = []

overall_maze_size_increases = []
overall_maze_complexity_increases = []

for i in range(len(pre_agent_amounts[0])):
    agent_amount = []

    avg_agent_size = []
    largest_agent_size = []
    smallest_agent_size = []

    overall_agent_size_increase = []

    maze_amount = []

    avg_maze_size = []
    largest_maze_size = []
    smallest_maze_size = []
    avg_maze_complexity = []
    largest_maze_complexity = []
    smallest_maze_complexity = []

    overall_maze_size_increase = []
    overall_maze_complexity_increase = []

    for j in range(len(pre_agent_amounts)):
        agent_amount.append(pre_agent_amounts[j][i])
        avg_agent_size.append(pre_avg_agent_sizes[j][i])
        largest_agent_size.append(pre_largest_agent_sizes[j][i])
        smallest_agent_size.append(pre_smallest_agent_sizes[j][i])

        overall_agent_size_increase.append(pre_overall_agent_size_increase[j][i])

        maze_amount.append(pre_maze_amounts[j][i])

        avg_maze_size.append(pre_avg_maze_sizes[j][i])
        largest_maze_size.append(pre_largest_maze_sizes[j][i])
        smallest_maze_size.append(pre_smallest_maze_sizes[j][i])
        avg_maze_complexity.append(pre_avg_maze_complexities[j][i])
        largest_maze_complexity.append(pre_largest_maze_complexities[j][i])
        smallest_maze_complexity.append(pre_smallest_maze_complexities[j][i])

        overall_maze_size_increase.append(pre_overall_maze_size_increase[j][i])
        overall_maze_complexity_increase.append(pre_overall_maze_complexity_increase[j][i])

    agent_amounts.append(agent_amount)

    avg_agent_sizes.append(avg_agent_size)
    largest_agent_sizes.append(largest_agent_size)
    smallest_agent_sizes.append(smallest_agent_size)

    overall_agent_size_increases.append(overall_agent_size_increase)

    maze_amounts.append(maze_amount)

    avg_maze_sizes.append(avg_maze_size)
    largest_maze_sizes.append(largest_maze_size)
    smallest_maze_sizes.append(smallest_maze_size)
    avg_maze_complexities.append(avg_maze_complexity)
    largest_maze_complexities.append(largest_maze_complexity)
    smallest_maze_complexities.append(smallest_maze_complexity)

    overall_maze_size_increases.append(overall_maze_size_increase)
    overall_maze_complexity_increases.append(overall_maze_complexity_increase)

for i in range(len(agent_amounts)):
    agent_amounts[i] = statistics.mean(agent_amounts[i])

    avg_agent_sizes[i] = statistics.mean(avg_agent_sizes[i])
    largest_agent_sizes[i] = statistics.mean(largest_agent_sizes[i])
    smallest_agent_sizes[i] = statistics.mean(smallest_agent_sizes[i])

    overall_agent_size_increases[i] = statistics.mean(overall_agent_size_increases[i])

    maze_amounts[i] = statistics.mean(maze_amounts[i])

    avg_maze_sizes[i] = statistics.mean(avg_maze_sizes[i])
    largest_maze_sizes[i] = statistics.mean(largest_maze_sizes[i])
    smallest_maze_sizes[i] = statistics.mean(smallest_maze_sizes[i])
    avg_maze_complexities[i] = statistics.mean(avg_maze_complexities[i])
    largest_maze_complexities[i] = statistics.mean(largest_maze_complexities[i])
    smallest_maze_complexities[i] = statistics.mean(smallest_maze_complexities[i])

    overall_maze_size_increases[i] = statistics.mean(overall_maze_size_increases[i])
    overall_maze_complexity_increases[i] = statistics.mean(overall_maze_complexity_increases[i])



font = {'family': 'arial',
        'color':  'black',
        'weight': 'normal',
        'size': 14,
        }



plt.style.use('seaborn-whitegrid')


def save_plot(data, y_label, file_name):
    x, y = [], []
    for i in range(len(data)):
        x.append(i)
        y.append(data[i])

    plt.plot(x, y)

    plt.ylabel(y_label, fontsize=14, fontdict=font)
    plt.xlabel("Generations", fontsize=14, fontdict=font)

    plt.savefig(file_name, bbox_inches="tight")
    plt.close()


def save_avg_max_min(avg, max, min, y_label, file_name):
    x, y = [], []
    for i in range(len(avg)):
        x.append(i)
        y.append(avg[i])

    plt.plot(x, y)

    x, y = [], []
    for i in range(len(max)):
        x.append(i)
        y.append(max[i])

    plt.plot(x, y)

    x, y = [], []
    for i in range(len(min)):
        x.append(i)
        y.append(min[i])

    plt.plot(x, y)

    plt.ylabel(y_label, fontsize=14, fontdict=font)
    plt.xlabel("Generations", fontsize=14, fontdict=font)

    plt.savefig(file_name, bbox_inches="tight")
    plt.close()


save_plot(agent_amounts, "Amount", "agent_amounts.png")
save_plot(maze_amounts, "Amount", "maze_amounts.png")

save_avg_max_min(avg_agent_sizes, largest_agent_sizes, smallest_agent_sizes, "Connections", "avg_max_min_agent_sizes.png")
save_avg_max_min(avg_maze_sizes, largest_maze_sizes, smallest_maze_sizes, "Size", "avg_max_min_maze_sizes.png")
save_avg_max_min(avg_maze_complexities, largest_maze_complexities, smallest_maze_complexities, "Junctures", "avg_max_min_maze_complexities.png")

save_plot(overall_maze_size_increases, "Increase/generations", "overall_maze_size_increases.png")
save_plot(overall_maze_complexity_increases, "Increase/generations", "overall_maze_complexity_increases.png")
