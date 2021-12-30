def parse(input: str):
    cmds = []
    for line in input.rstrip().split("\n"):
        parts = line.split(" ")
        cmds.append(
            {
                "state": parts[0],
                "cube": tuple([tuple(map(int, coord.split("=")[1].split(".."))) for coord in parts[1].split(",")]),
            }
        )
    return cmds


def isin(subcube, cube):
    return (
        subcube[0][0] >= cube[0][0]
        and subcube[0][1] <= cube[0][1]
        and subcube[1][0] >= cube[1][0]
        and subcube[1][1] <= cube[1][1]
        and subcube[2][0] >= cube[2][0]
        and subcube[2][1] <= cube[2][1]
    )


def is_before(range1, range2):
    return range1[0] <= range2[0]


def is_overlapping(range1, range2):
    if is_before(range1, range2):
        return range1[1] >= range2[0]
    else:
        return range2[1] >= range1[0]


def overlap_ranges(range1, range2):
    """
    Assumes that range1 <= range2.
    Always returns non-overlapping regions to make counting active cubes easier.
    """
    if is_overlapping(range1, range2) and range1 != range2:
        if range1[0] == range2[0]:
            intersection = min(range1[1], range2[1])
            endpoint = max(range1[1], range2[1])
            return {(range1[0], intersection), (intersection + 1, endpoint)}
        elif range1[1] == range2[1]:
            return {(range1[0], range2[0]), (range2[0] + 1, range2[1])}
        else:
            return {(range1[0], range2[0] - 1), (range2[0], range1[1]), (range1[1] + 1, range2[1])}
    else:
        return {range1, range2}


def is_valid(cube):
    return cube[0][0] <= cube[0][1] and cube[1][0] <= cube[1][1] and cube[2][0] <= cube[2][1]


def union(cube1, cube2) -> set:
    cubes = set()

    # let's keep them ordered for simplicity
    if is_before(cube2, cube1):
        cube1, cube2 = cube2, cube1

    xs = overlap_ranges(cube1[0], cube2[0])
    ys = overlap_ranges(cube1[1], cube2[1])
    zs = overlap_ranges(cube1[2], cube2[2])

    for (x1, x2) in xs:
        for (y1, y2) in ys:
            for (z1, z2) in zs:
                new_cube = ((x1, x2), (y1, y2), (z1, z2))

                if is_valid(new_cube) and (isin(new_cube, cube1) or isin(new_cube, cube2)):
                    cubes.add(new_cube)

    return cubes


def difference(cube1, cube2) -> set:
    cubes = set()

    include_first, include_second = lambda x: x, lambda x: not x
    # let's keep them ordered for simplicity
    if is_before(cube2, cube1):
        cube1, cube2 = cube2, cube1
        include_first, include_second = include_second, include_first

    xs = overlap_ranges(cube1[0], cube2[0])
    ys = overlap_ranges(cube1[1], cube2[1])
    zs = overlap_ranges(cube1[2], cube2[2])

    for (x1, x2) in xs:
        for (y1, y2) in ys:
            for (z1, z2) in zs:
                new_cube = ((x1, x2), (y1, y2), (z1, z2))

                if is_valid(new_cube) and (
                    include_first(isin(new_cube, cube1)) and include_second(isin(new_cube, cube2))
                ):
                    cubes.add(new_cube)

    return cubes


def execute(cmds):
    # first command is always "on"
    active = {cmds[0]["cube"]}

    for cmd in cmds[1:]:
        cube2 = cmd["cube"]

        if cmd["state"] == "on":
            for cube1 in active:
                active = active.union(union(cube1, cube2))
        else:
            new_active = set()

            for cube1 in active:
                still_on = difference(cube1, cube2)
                if still_on:
                    new_active = new_active.union(still_on)

            active = new_active

    return active


def subcubes_count(cube):
    return (cube[0][1] - cube[0][0] + 1) * (cube[1][1] - cube[1][0] + 1) * (cube[2][1] - cube[2][0] + 1)


# TODO: this double counts the border cubes if they are in several regions
def subcubes_count_all(cubes: set):
    count = 0
    for cube in cubes:
        count += subcubes_count(cube)
    return count


def expand(cubes):
    all_cubes = set()

    for cube in cubes:
        for x in range(cube[0][0], cube[0][1] + 1):
            for y in range(cube[1][0], cube[1][1] + 1):
                for z in range(cube[2][0], cube[2][1] + 1):
                    all_cubes.add((x, y, z))

    return all_cubes


input = """on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
"""

cmds = parse(input)
active = execute(cmds)
len(expand(active))

cube1 = cmds[0]["cube"]
cube2 = cmds[1]["cube"]
cube3 = ((10, 12), (100, 150), (10, 12))
active1 = union(cube1, cube2)
active2 = difference(active1)


cube1 = ((11, 12), (13, 13), (11, 12))
cube2 = ((9, 11), (9, 11), (9, 11))

active = [[], [], []]

for cmd in cmds:
    if cmd["state"] == "on":
        for dim, coord in enumerate(cmd["coords"]):
            active[dim].append(coord)
    elif cmd["state"] == "off":
        pass


# 2 ons: x = 10..13, y = 10..13, z = 10..13 # union
# off: x = 12..13, y = 12..13, 12..13 # difference
# on: x = [10..10, 12..13], ... # union
