from pprint import pprint


def ds(x, y):
    total = 0
    for d in (str(x) + str(y)):
        total += int(d)
    return total


def max_line_move(x0, y0):
    for i in range(x0, 10000):
        total = ds(i, y0)
        if total > 25:
            return i


def make_visualisation(file_path: str = 'confusion_matrix.png'):
    """
    Создает файл с наглядным распределением сумм цифр координат.
    :param file_path:
    :return:
    """
    total_count = 0
    x0, y0 = 1000, 1000
    max_x = max_y = max_line_move(1000, 1000)
    conf_matrix = []
    for x in range(x0, max_x):
        conf_matrix.append([])
        for y in range(y0, max_y):
            conf_matrix[-1].append(ds(x, y))
            if ds(x, y) <= 25:
                total_count += 1

    import pandas as pd
    import matplotlib.pyplot as plt
    import seaborn as sns

    conf_matrix_df = pd.DataFrame(conf_matrix)

    sns.heatmap(conf_matrix_df,
                cmap='Greens',
                vmax=26)

    plt.savefig('confusion_matrix.png', format='png', dpi=1500)


if __name__ == "__main__":
    make_visualisation()
