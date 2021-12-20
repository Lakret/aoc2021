using LinearAlgebra
using Pipe

function parse_input(input)::Dict{Int32,Matrix{Int32}}
    reports = @pipe input |> chomp |> split(_, "\n\n") |> map(report -> split(report, "\n"), _)
    reports = map(
        report -> begin
            scanner_id = @pipe replace(report[1], "-" => "") |> replace(_, "scanner" => "") |> replace(_, " " => "")
            scanner_id = parse(Int32, scanner_id)

            beacons = map(coords -> map(c -> parse(Int32, c), split(coords, ",")), report[2:end])
            beacons = hcat(beacons...) |> permutedims

            scanner_id => beacons
        end,
        reports
    )

    Dict(reports)
end

"""
Finds matching beacon coordinates, using length between beacons (coordinate system independent)
to find matches.

Returns a dictonary of matching coordinates with keys as the left scanner's perspective coords,
and values as the right scanner's perspective coords.
"""
function find_matching(r1::Matrix{Int32}, r2::Matrix{Int32})::Dict{Vector{Int32},Vector{Int32}}
    same = Dict()

    for r1_i = 1:size(r1)[1], r1_j = 1:size(r1)[1], r2_i = 1:size(r2)[1], r2_j = 1:size(r2)[1]
        if r1_i < r1_j && r2_i < r2_j
            s1b1, s1b2, s2b1, s2b2 = r1[r1_i, :], r1[r1_j, :], r2[r2_i, :], r2[r2_j, :]
            if norm(s1b1 - s1b2) == norm(s2b1 - s2b2)
                if haskey(same, s1b1)
                    # we can assume that if the same r1_i came up again, the beacon
                    # it really corresponds to will be already present in the set of possibilities
                    # for it
                    same[s1b1] = intersect(same[s1b1], [s2b1, s2b2])
                else
                    # we don't know if r1_i corresponds to the first or second beacon
                    same[s1b1] = [s2b1, s2b2]
                end

                if haskey(same, s1b2)
                    same[s1b2] = intersect(same[s1b2], [s2b1, s2b2])
                else
                    same[s1b2] = [s2b1, s2b2]
                end
            end
        end
    end

    Dict(entry[1] => entry[2][1] for entry in same if length(entry[2]) == 1)
end

"""
Returns right scanner coords according to the left scanner's perspective,
as a matrix of `[offset rdim sign]`.

`same` should be a dictionary of matching coordinates with left scanner's perspective
as keys, and right scanner's perspective as values.
"""
function find_translation_matrix(same::Dict{Vector{Int32},Vector{Int32}})::Matrix{Int32}
    l, r = [], []
    for entry in same
        push!(l, entry[1])
        push!(r, entry[2])
    end

    l = hcat(l...) |> permutedims
    r = hcat(r...) |> permutedims

    r_coords = [0 1 1; 0 2 1; 0 3 1]

    # if a coordinate matches, it should be the same for all matching points
    for ldim = 1:3, rdim = 1:3, sign in [1, -1]
        #
        uniques = Set(l[:, ldim] .- sign .* r[:, rdim])
        if length(uniques) == 1
            r_coords[ldim, :] = [pop!(uniques), rdim, sign]
        end
    end

    r_coords
end

"""
Translates coordinates from right scanner's view `coords` to origin scanner's view,
using the translation matrix `to_scanner` (returned from `find_scanner_translation_matrix`).
"""
function translate_coords(coords, right_translation; left_translation = [0 1 1; 0 2 1; 0 3 1])
    new_coords = [0, 0, 0]

    for dim = 1:3
        offset, rdim, sign = right_translation[dim, 1], right_translation[dim, 2], right_translation[dim, 3]
        new_coords[dim] = sign * coords[rdim] + offset
    end

    new_coords
end

function translate_translation_matrix(m::Matrix{Int32}, right_translation)::Matrix{Int32}
    m2 = copy(m)
    m2[:, 1] = translate_coords(m[:, 1], right_translation)
    m2[:, 3] = m2[:, 3] .* right_translation[:, 3]
    m2
end

same01 = find_matching(test_reports[0], test_reports[1])
same14 = find_matching(test_reports[1], test_reports[4])
same42 = find_matching(test_reports[4], test_reports[2])
same13 = find_matching(test_reports[1], test_reports[3])

ori1 = find_translation_matrix(same01)
ori4_from1 = find_translation_matrix(same14)
ori4 = translate_translation_matrix(ori4_from1, ori1)
ori2_from4 = @pipe find_translation_matrix(same42) |> translate_translation_matrix(_, ori4)
ori2 = @pipe find_translation_matrix(same42) |> translate_translation_matrix(_, ori4)
ori3 = @pipe find_translation_matrix(same13) |> translate_translation_matrix(_, ori1)

fixed_keys = collect(keys(same))
map(coords -> translate_coords(coords, ori1), [same[k] for k in fixed_keys]) == fixed_keys
map(coords -> translate_coords(coords, ori2), [same[k] for k in fixed_keys]) == fixed_keys
map(coords -> translate_coords(coords, ori4_from0), values(same41))

all_coords = Set() # should be 79, but we get more and not all test ones are here
for c in eachrow(test_reports[0])
    push!(all_coords, c)
end
for c in eachrow(test_reports[1])
    push!(all_coords, translate_coords(c, ori1))
end
for c in eachrow(test_reports[2])
    push!(all_coords, translate_coords(c, ori2))
end
for c in eachrow(test_reports[3])
    push!(all_coords, translate_coords(c, ori3))
end
for c in eachrow(test_reports[4])
    push!(all_coords, translate_coords(c, ori4))
end

expected_all = [
    [-892, 524, 684],
    [-876, 649, 763],
    [-838, 591, 734],
    [-789, 900, -551],
    [-739, -1745, 668],
    [-706, -3180, -659],
    [-697, -3072, -689],
    [-689, 845, -530],
    [-687, -1600, 576],
    [-661, -816, -575],
    [-654, -3158, -753],
    [-635, -1737, 486],
    [-631, -672, 1502],
    [-624, -1620, 1868],
    [-620, -3212, 371],
    [-618, -824, -621],
    [-612, -1695, 1788],
    [-601, -1648, -643],
    [-584, 868, -557],
    [-537, -823, -458],
    [-532, -1715, 1894],
    [-518, -1681, -600],
    [-499, -1607, -770],
    [-485, -357, 347],
    [-470, -3283, 303],
    [-456, -621, 1527],
    [-447, -329, 318],
    [-430, -3130, 366],
    [-413, -627, 1469],
    [-345, -311, 381],
    [-36, -1284, 1171],
    [-27, -1108, -65],
    [7, -33, -71],
    [12, -2351, -103],
    [26, -1119, 1091],
    [346, -2985, 342],
    [366, -3059, 397],
    [377, -2827, 367],
    [390, -675, -793],
    [396, -1931, -563],
    [404, -588, -901],
    [408, -1815, 803],
    [423, -701, 434],
    [432, -2009, 850],
    [443, 580, 662],
    [455, 729, 728],
    [456, -540, 1869],
    [459, -707, 401],
    [465, -695, 1988],
    [474, 580, 667],
    [496, -1584, 1900],
    [497, -1838, -617],
    [527, -524, 1933],
    [528, -643, 409],
    [534, -1912, 768],
    [544, -627, -890],
    [553, 345, -567],
    [564, 392, -477],
    [568, -2007, -577],
    [605, -1665, 1952],
    [612, -1593, 1893],
    [630, 319, -379],
    [686, -3108, -505],
    [776, -3184, -501],
    [846, -3110, -434],
    [1135, -1161, 1235],
    [1243, -1093, 1063],
    [1660, -552, 429],
    [1693, -557, 386],
    [1735, -437, 1738],
    [1749, -1800, 1813],
    [1772, -405, 1572],
    [1776, -675, 371],
    [1779, -442, 1789],
    [1780, -1548, 337],
    [1786, -1538, 337],
    [1847, -1591, 415],
    [1889, -1729, 1762],
    [1994, -1805, 1792]
]


map(coords -> translate_coords(coords, ori1), values(same01))
map(coords -> translate_coords(coords, ori1), values(same14))


for x in keys(test_reports), y in keys(test_reports)
    if x < y
        same = find_matching(test_reports[x], test_reports[y])
        if length(same) >= 12
            println("$x, $y => $same")
        end
    end
end





for x in keys(reports), y in keys(reports)
    if x < y
        same = find_matching(reports[x], reports[y])
        if length(same) >= 12
            println("$x, $y")
        end
    end
end


find_matching(reports[5], reports[32])


find_matching(test_reports[0], test_reports[1])

@pipe values(reports) |> map(size, _)

m1 = find_matching(test_reports[0], test_reports[1])
find_matching(test_reports[1], test_reports[4])
find_matching(test_reports[1], test_reports[1])

s = Set()
for i = 1:66
    push!(s, (m1[i, 1], m1[i, 2]))
end


# TODO:
# first, we can determine matching scanners by ignoring rotations, and looking at each scanner A and another scanner B
# if we find matchign norms between any two points p1 and p2, we know that those points are matching - need to check if there are matching norms
# 90% of distances are unique


input = read("d19_input", String)
reports = parse_input(input)

small_test_input = """
--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1
"""

small_test_reports = parse_input(small_test_input)

test_input = """
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"""

test_reports = parse_input(test_input)
