# -*- coding: utf-8 -*-
import time


class Step:
    def __init__(self, pos, new):
        """
        Класс хранит информацию о шаге:
        pos - текущее положение на поле, кортеж с координатами
        new - флаг, указывающий, был ли данный шаг уже пройден
        """
        self.pos = pos
        self.new = new


class Field:
    def __init__(self, start_pos):
        """
        Класс, представляющий игровое поле:
        start_pos - начальное положение на поле, кортеж с координатами
        front_line - список шагов, которые могут быть сделаны из текущего положения на поле
        steps - список всех сделанных шагов
        """
        self.front_line = [Step(pos=start_pos, new=True)]
        self.steps = []

    def sum_of_literal_digits(self, pos):
        """
        Метод для вычисления суммы цифр координат переданной позиции
        """
        x = str(int(pos[0]))
        y = str(int(pos[1]))
        xy = x + y
        s = sum(int(c) for c in xy)
        return float(s)

    def move_to(self, pos):
        """
        Метод для перемещения к новой позиции на поле:
        pos - позиция, куда нужно переместиться, кортеж с координатами
        """
        free_space = all(s.pos != pos for s in self.front_line)  # проверяем, что на новой позиции еще нет шага
        can_step_by_sum = self.sum_of_literal_digits(pos) < 26  # проверяем, что сумма цифр координат меньше 26

        if free_space and can_step_by_sum:
            # если на новой позиции нет шага и сумма цифр координат меньше 26, то добавляем новый шаг
            self.front_line.append(Step(pos=pos, new=True))

    def expand(self):
        """
        Метод для расширения поля:
        - перебираем все шаги в front_line
        - для каждого нового шага добавляем шаги вправо и вверх
        - помечаем текущий шаг, как пройденный и добавляем его в steps
        """
        have_new = False
        for i in range(len(self.front_line)):
            if self.front_line[i].new:
                have_new = True

                right_pos = (self.front_line[i].pos[0] + 1, self.front_line[i].pos[1])
                self.move_to(right_pos)

                up_pos = (self.front_line[i].pos[0], self.front_line[i].pos[1] + 1)
                self.move_to(up_pos)

                self.front_line[i].new = False
                self.steps.append(self.front_line[i])

        self.front_line = [s for s in self.front_line if s.new]  # удаляем пройденные шаги из front_line

        return have_new


def main():
    time_start = time.time()
    field = Field(start_pos=(1000, 1000))

    while True:
        active = field.expand()

        if not active:
            steps = len(field.steps)
            print("Steps taken: {}".format(steps))
            print("Elapsed time: {:.1f}".format(time.time() - time_start))
            return steps


if __name__ == "__main__":
    assert main() == 148848
