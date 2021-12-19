def add(x, y):
    return [x, y]


assert add([1, 2], [[3, 4], 5]) == [[1, 2], [[3, 4], 5]]


def walk(num):
    if type(num) == list:
        walk(num[0])
        walk(num[1])
    else:
        print(num)


def walk_iter(num):
    stack = [num]

    while stack:
        num = stack.pop()

        if type(num) == list:
            stack.append(num[1])
            stack.append(num[0])
        else:
            print(num)


from copy import deepcopy

# TODO: maybe we need another stack with paths + action to apply for expand?
def explode(num):
    # num = deepcopy(num)
    stack = [(num, 0)]
    residual = None

    while stack:
        (num, level) = stack.pop()

        if type(num) == list:
            if level == 3:
                if type(num[0]) == int and type(num[1]) == list and type(num[1][0]) == int and type(num[1][1]) == int:
                    num[0] += num[1][0]
                    residual = num[1][1]
                    num[1] = 0

                elif type(num[0]) == list and type(num[0][0]) == int and type(num[0][1]) == int and type(num[1]) == int:
                    num[1] += num[0][1]
                    num[0] = 0

            stack.append((num[1], level + 1))
            stack.append((num[0], level + 1))
        else:
            if residual:
                num = 0
                residual = None

            print((level, num))


x = [[[[[9, 8], 1], 2], 3], 4]
explode(x)

test = [[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]
num = [[6, [5, [4, [3, 2]]]], 1]
