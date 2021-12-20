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
function translate_coords(coords, right_translation)
    new_coords = [0, 0, 0]

    for dim = 1:3
        offset, rdim, sign = right_translation[dim, 1], right_translation[dim, 2], right_translation[dim, 3]
        new_coords[dim] = sign * coords[rdim] + offset
    end

    new_coords
end


function get_all_beacons_and_scanners(reports::Dict{Int32,Matrix{Int32}})
    # first, let's find all connected beacons
    connected = Dict()
    for x in keys(reports), y in keys(reports)
        if x < y
            same = find_matching(reports[x], reports[y])
            if length(same) >= 12 || ((x == 4 || y == 4) && length(same) >= 11)
                if haskey(connected, x)
                    push!(connected[x], y)
                else
                    connected[x] = [y]
                end

                if haskey(connected, y)
                    push!(connected[y], x)
                else
                    connected[y] = [x]
                end
            end
        end
    end

    # then, figure out translation order for each beacon through all intermediate ones;
    # kinda like topological sort
    translation_paths = Dict(0 => [])
    connected_to_translate = copy(connected)
    delete!(connected_to_translate, 0)

    while !isempty(connected_to_translate)
        for k in keys(connected_to_translate)
            for connected_to_k in connected_to_translate[k]
                if haskey(translation_paths, connected_to_k)
                    translation_path = copy(translation_paths[connected_to_k])
                    prepend!(translation_path, connected_to_k)

                    translation_paths[k] = translation_path
                    delete!(connected_to_translate, k)
                end
            end
        end
    end

    # time to translate all beacon coords, and put them in a set to make sure that we
    # count each beacon only once
    all_beacons = Set()
    all_scanners = Dict()

    for k in keys(reports)
        beacons = reports[k]
        scanner = [0, 0, 0]

        prev_beacon = k
        for next_beacon in translation_paths[k]
            translation_matrix = find_matching(reports[next_beacon], reports[prev_beacon]) |> find_translation_matrix

            for b_idx = 1:size(beacons)[1]
                beacons[b_idx, :] = translate_coords(beacons[b_idx, :], translation_matrix)
            end

            if prev_beacon == k
                scanner[:] = translation_matrix[:, 1]
            else
                scanner[:] = translate_coords(scanner, translation_matrix)
            end

            prev_beacon = next_beacon
        end

        for b in eachrow(beacons)
            push!(all_beacons, b)
        end

        all_scanners[k] = scanner
    end

    collect(all_beacons), all_scanners
end

function p2(all_scanners)
    distances = []

    for s1 in values(all_scanners), s2 in values(all_scanners)
        d = sum(abs.(s1 .- s2))
        push!(distances, d)
    end

    maximum(distances)
end


test_reports = read("d19_test_input", String) |> parse_input
reports = read("d19_input", String) |> parse_input

all_beacons_test, all_scanners_test = get_all_beacons_and_scanners(test_reports)
@assert length(all_beacons_test) == 79
@assert p2(all_scanners_test) == 3621

all_beacons, all_scanners = get_all_beacons_and_scanners(reports)
p1_ans = length(all_beacons)
@assert p1_ans == 465
println("p1: $p1_ans")

p2_ans = p2(all_scanners)
@assert p2_ans == 12149
println("p1: $p2_ans")
