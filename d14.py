from collections import Counter


def parse(input: str):
    lines = input.rstrip().split("\n")
    template = lines[0]
    rules = {line.split(" -> ")[0]: line.split(" -> ")[1] for line in lines[2:]}
    return template, rules


def run(counts, rules):
    new_counts = {}
    for (pair, count) in counts.items():
        if pair in rules:
            inserted_letter = rules[pair]
            pair1, pair2 = pair[0] + inserted_letter, inserted_letter + pair[1]
            new_counts[pair1] = new_counts.get(pair1, 0) + count
            new_counts[pair2] = new_counts.get(pair2, 0) + count
    return new_counts


def solve(template, rules, steps=10):
    counts = Counter([template[idx - 1] + template[idx] for idx in range(1, len(template))])

    for _ in range(steps):
        counts = run(counts, rules)

    final_counts = {}
    for pair, count in counts.items():
        final_counts[pair[0]] = final_counts.get(pair[0], 0) + count
    final_counts[template[-1]] = final_counts.get(template[-1], 0) + 1

    return max(final_counts.values()) - min(final_counts.values())


if __name__ == "__main__":
    test_input = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"""

    test_template, test_rules = parse(test_input)

    with open("d14_input", "rt") as f:
        template, rules = parse(f.read())

    assert solve(test_template, test_rules) == 1588
    p1_ans = solve(template, rules)
    assert p1_ans == 2408
    print(f"p1: {p1_ans}")

    assert solve(test_template, test_rules, steps=40) == 2188189693529
    p2_ans = solve(template, rules, steps=40)
    assert p2_ans == 2651311098752
    print(f"p2: {p2_ans}")
