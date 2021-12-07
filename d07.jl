using Pipe

cost(input, pos) = sum(abs.(input .- pos))
min_cost(input; cost_fun = cost) = [cost_fun(input, pos) for pos = minimum(input):maximum(input)] |> minimum

function cost2(input, pos)::Int64
    steps = abs.(input .- pos)
    costs = @. steps * (steps + 1) / 2
    floor(sum(costs))
end

test_input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
input = @pipe read("d07_input", String) |> chomp |> split(_, ",") |> map(x -> parse(Int32, x), _)

@assert min_cost(test_input) == 37

@time p1_ans = min_cost(input)
@assert p1_ans == 341558
println("p1_ans = $p2_ans")

@assert cost2(test_input, 2) == 206
@assert min_cost(test_input, cost_fun = cost2) == 168

@time p2_ans = min_cost(input, cost_fun = cost2)
@assert p2_ans == 93214037
println("p2 = $p2_ans")
