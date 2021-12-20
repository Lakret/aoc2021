from copy import deepcopy


def add(x, y):
    return [x, y]


t = [[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]
z = as_zipper(t)

first_expand_node = move_right(move_right(move_right(move_left(z))))
assert len(first_expand_node[1]) == 4
back_up = move_up(move_up(move_up(move_up(first_expand_node))))
assert back_up[1] == []  # zipper is empty
assert back_up[0] == t  # tree is fully reconstructed
move_up(back_up)


def expand_focused(zipper):
    current = deepcopy(zipper)
    (tree, zipper_part) = deepcopy(zipper)

    left_carry, right_carry = tree[0], tree[1]

    # find left neighbor to modify
    if zipper_part[-1][0] == "r":
        # if the expanded node is the right child,
        # left neighbor is just the left child of the same parent
        left_neighbour = move_left(move_up(current))
        moves = ["u", "l"]
        while type(left_neighbour[0]) != int:
            left_neighbour = move_right(left_neighbour)
            moves.append("r")

        left_neighbour = (left_neighbour[0] + left_carry, left_neighbour[1])
    else:
        # if the expanded node is the left child,
        # move up twice
        left_neighbour = move_left(move_up(move_up(current)))
        moves = ["u", "u", "l"]

        while type(left_neighbour[0]) != int:
            left_neighbour = move_right(left_neighbour)
            moves.append("r")

        left_neighbour = (left_neighbour[0] + left_carry, left_neighbour[1])

    # go back up
    moves.reverse()
    for move in moves:
        if move == "l" or move == "r":
            left_neighbour = move_up(left_neighbour)
        else:
            left_neighbour = move_right(left_neighbour)

    current = left_neighbour

    # find right neighbor to modify
    if zipper_part[-1][0] == "l":
        # if the expanded node is the left child,
        # right neighbor is just the right child of the same parent
        right_neighbour = move_right(move_up(current))
        moves = ["u", "r"]
        while type(right_neighbour[0]) != int:
            right_neighbour = move_left(right_neighbour)
            moves.append("l")

        right_neighbour = (right_neighbour[0] + right_carry, right_neighbour[1])
    else:
        # if the expanded node is the right child,
        # move up twice
        right_neighbour = move_right(move_up(move_up(current)))
        moves = ["u", "u", "r"]

        while type(right_neighbour[0]) != int:
            right_neighbour = move_left(right_neighbour)
            moves.append("l")

        print(right_neighbour)

        right_neighbour = (right_neighbour[0] + right_carry, right_neighbour[1])

    # go back up
    moves.reverse()
    for move in moves:
        if move == "l" or move == "r":
            right_neighbour = move_up(right_neighbour)
        else:
            right_neighbour = move_left(right_neighbour)

    # replace expanded with 0
    (_, zipper_part) = right_neighbour
    return (0, zipper_part)


def as_zipper(tree):
    return (tree, [])


def move_left(zipper):
    (tree, zipper) = deepcopy(zipper)

    if type(tree) == list:
        zipper.append(("l", tree[1]))
        return (tree[0], zipper)
    else:
        return None


def move_right(zipper):
    (tree, zipper) = deepcopy(zipper)

    if type(tree) == list:
        zipper.append(("r", tree[0]))
        return (tree[1], zipper)
    else:
        return None


def move_up(zipper):
    (tree, zipper) = deepcopy(zipper)

    if len(zipper) > 0:
        (last_move_dir, opposite_branch) = zipper[-1]
        zipper = zipper[:-1]

        if last_move_dir == "l":
            tree = [tree, opposite_branch]
            return (tree, zipper)
        elif last_move_dir == "r":
            tree = [opposite_branch, tree]
            return (tree, zipper)
        else:
            raise Exception("Unknown move: {}.", last_move_dir)
    else:
        return None


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
