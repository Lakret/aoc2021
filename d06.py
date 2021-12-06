from collections import Counter
from copy import deepcopy


def parse(input):
    return Counter(map(int, input.split(",")))


def step(counts):
    new_counts = Counter()

    new_counts[6], new_counts[8] = counts[0], counts[0]

    for days in range(1, 9):
        new_counts[days - 1] = new_counts[days - 1] + counts[days]

    return new_counts


def run(counts, time=80):
    counts = deepcopy(counts)

    for _ in range(time):
        counts = step(counts)

    return sum(counts.values())


if __name__ == "__main__":
    test_input = "3,4,3,1,2"
    test_counts = parse(test_input)

    assert run(test_counts) == 5934
    assert run(test_counts, 256) == 26984457539

    with open("d06_input", "rt") as f:
        input = f.read().strip()

    counts = parse(input)
    p1_ans = run(counts)
    assert p1_ans == 362_346
    print(f"p1: {p1_ans}")

    p2_ans = run(counts, 256)
    assert p2_ans == 1639643057051
    print(f"p2: {p2_ans}")
