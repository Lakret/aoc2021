def parse(input):
    steps = []
    for step in input.strip().split("\n"):
        direction, value = step.split(" ")
        steps.append((direction, int(value)))
    return steps


def run(steps):
    x, y = 0, 0
    for (direction, value) in steps:
        if direction == "forward":
            x += value
        elif direction == "up":
            y -= value
        elif direction == "down":
            y += value
        else:
            raise f"unknown direction {direction}"
    return x, y


def p1(input):
    x, y = run(parse(input))
    return x * y


def run2(steps):
    x, y, aim = 0, 0, 0
    for (direction, value) in steps:
        if direction == "forward":
            x += value
            y += value * aim
        elif direction == "up":
            aim -= value
        elif direction == "down":
            aim += value
        else:
            raise f"unknown direction {direction}"
    return x, y


def p2(input):
    x, y = run2(parse(input))
    return x * y


if __name__ == "__main__":
    test_input = """forward 5
down 5
forward 8
up 3
down 8
forward 2
"""
    assert p1(test_input) == 150

    with open("d02_input", "rt") as f:
        input = f.read()

    p1_ans = p1(input)
    assert p1_ans == 1635930
    print(f"p1: {p1_ans}")

    assert p2(test_input) == 900
    p2_ans = p2(input)
    assert p2_ans == 1781819478
    print(f"p2: {p2_ans}")
