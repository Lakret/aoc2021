from math import floor

opening_braces = {"{", "(", "[", "<"}
brace_pairs = {"{": "}", "(": ")", "[": "]", "<": ">"}
mismatch_scores = {")": 3, "]": 57, "}": 1197, ">": 25137}
closing_scores = {")": 1, "]": 2, "}": 3, ">": 4}


def parse(input: str):
    return input.rstrip().splitlines()


def p1(lines):
    return sum([mismatch_scores[m[0]] for m in [find_mismatch(line) for line in lines] if m[0]])


def p2(lines):
    scores = [complete_score(line) for line in lines if not find_mismatch(line)[0]]
    scores.sort()
    return scores[floor(len(scores) / 2)]


def find_mismatch(test_line):
    stack = []

    for ch in test_line:
        if ch in opening_braces:
            stack.append(ch)
        else:
            expected_openning = stack.pop()
            if brace_pairs[expected_openning] != ch:
                return (ch, stack)

    return (None, stack)


def complete_score(line):
    (_, stack) = find_mismatch(line)
    stack.reverse()

    s = 0
    for ch in stack:
        closing_ch = brace_pairs[ch]
        s = s * 5 + closing_scores[closing_ch]

    return s


if __name__ == "__main__":
    test_input = """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"""

    test_lines = parse(test_input)

    with open("d10_input", "rt") as f:
        input = f.read()

    lines = parse(input)

    assert find_mismatch("{([(<{}[<>[]}>{[]{[(<()>")[0] == "}"
    assert find_mismatch("[[<[([]))<([[{}[[()]]]")[0] == ")"
    assert find_mismatch("[{[{({}]{}}([{[{{{}}([]")[0] == "]"
    assert find_mismatch("[<(<(<(<{}))><([]([]()")[0] == ")"
    assert find_mismatch("<{([([[(<>()){}]>(<<{{")[0] == ">"
    assert find_mismatch("[({(<(())[]>[[{[]{<()<>>")[0] is None

    assert p1(test_lines) == 26397

    p1_ans = p1(lines)
    assert p1_ans == 374061
    print(f"p1: {p1_ans}")

    assert complete_score("[({(<(())[]>[[{[]{<()<>>") == 288957

    assert p2(test_lines) == 288957

    p2_ans = p2(lines)
    assert p2_ans == 2116639949
    print(f"p2: {p2_ans}")
