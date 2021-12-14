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

from collections import Counter


def parse(input: str):
    lines = input.rstrip().split("\n")
    template = lines[0]
    rules = {line.split(" -> ")[0]: line.split(" -> ")[1] for line in lines[2:]}
    return template, rules


template, rules = parse(test_input)
counts = Counter([template[idx - 1] + template[idx] for idx in range(1, len(template))])
run2(run2(run2(run2(counts, rules), rules), rules), rules)


def run2(counts, rules):
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
        counts = run2(counts, rules)

    final_counts = {}
    for pair, count in counts.items():
        final_counts[pair[0]] = final_counts.get(pair[0], 0) + count
        # final_counts[pair[1]] = final_counts.get(pair[1], 0) + count

    final_counts[template[-1]] = final_counts.get(template[-1], 0) + 1

    return max(final_counts.values()) - min(final_counts.values())


def run(template: str, rules: dict):
    pairs = [template[idx - 1] + template[idx] for idx in range(1, len(template))]
    triples = [pair[0] + rules[pair] + pair[1] if rules.get(pair) else pair for pair in pairs]
    polymer = triples[0]
    for triple in triples[1:]:
        polymer += triple[1:]
    return polymer


# TODO: can we just have a dict with counts running through? do we ever add new pairs? can we track adding new pairs?
def p1(template, rules, steps=10):
    for _ in range(steps):
        template = run(template, rules)
    counts = list(Counter(template).values())
    return max(counts) - min(counts)


with open("d14_input", "rt") as f:
    template, rules = parse(f.read())

p1(template, rules)
