using Pkg
Pkg.add(["Pipe", "StatsBase"])

using Pipe
using StatsBase


function parse_report(input)
    report = @pipe input |> chomp |> split(_, "\n")
    report = hcat(collect.(report)...) |> permutedims
    map(x -> parse(Int32, x), report)
end

binary_mode(report) = join(mode.(eachcol(report)))
rev(binary) = join(replace(collect(binary), '1' => '0', '0' => '1'))

γ(report) = parse(Int32, binary_mode(report), base = 2)
ϵ(report) = parse(Int32, report |> binary_mode |> rev, base = 2)

power_consumption(report) = γ(report) * ϵ(report)


function ratings(report; oxygen = true)
    og_rating = collect(eachrow(copy(report)))

    cols = size(og_rating[1])[1]
    for col = 1:cols
        og_matrix = hcat(og_rating...) |> permutedims

        most_common = mode(og_matrix[:, col])
        most_common_count = count(==(most_common), og_matrix[:, col])

        if size(og_matrix)[1] - most_common_count == most_common_count
            most_common = 1
        end

        if oxygen
            pred = x -> x[col] == most_common
        else
            pred = x -> x[col] != most_common
        end

        og_rating = collect(filter(pred, og_rating))
        if size(og_rating)[1] == 1
            break
        end
    end

    parse(Int32, join.(og_rating)[1], base = 2)
end

p2(report) = ratings(report) * ratings(report, oxygen = false)


test_input = """
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""
test_report = parse_report(test_input)
@assert power_consumption(test_report) == 198

input = read("d03_input", String)
report = parse_report(input)

p1_ans = power_consumption(report)
@assert p1_ans == 1540244
println("p1: $p1_ans")

@assert ratings(test_report) == 23
@assert ratings(test_report, oxygen = false) == 10

p2_ans = p2(report)
@assert p2_ans == 4203981
println("p2: $p2_ans")
