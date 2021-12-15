using Pipe
using DataStructures

parse_input(input)::Matrix{UInt16} =
    @pipe input |> chomp |> split(_, "\n") |> map(collect, _) |> hcat(_...) |> permutedims |> parse.(UInt16, _)

function dijkstra(graph::Matrix{UInt16})
    target = maximum(keys(graph))
    current = CartesianIndex(1, 1)
    distances = Dict(current => graph[current])
    didx = CartesianIndex.([(-1, 0), (0, -1), (1, 0), (0, 1)])

    unvisited = PriorityQueue()
    for idx in keys(graph)
        unvisited[idx] = Inf
    end

    while true
        for idx in didx
            next = current + idx

            if next ∈ keys(graph) && next ∈ keys(unvisited)
                new_distance = distances[current] + graph[next]

                if next == target
                    return new_distance - distances[CartesianIndex(1, 1)]
                else
                    distances[next] = min(get(distances, next, Inf), new_distance)
                    unvisited[next] = distances[next]
                end
            end
        end

        current = dequeue!(unvisited)
    end
end


function enlarge(graph::Matrix{UInt16})::Matrix{UInt16}
    rows = []

    for drow = 1:5
        row = []

        for dcol = 1:5
            tile = (graph .- 1 .+ (drow - 1) .+ (dcol - 1)) .% 9 .+ 1
            tile[tile.==0] .= 1

            push!(row, tile)
        end

        push!(rows, hcat(row...))
    end

    return vcat(rows...)
end



test_input = """
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"""

test_graph = parse_input(test_input)

input = read("d15_input", String);
graph = parse_input(input)

@assert dijkstra(test_graph) == 40

p1_ans = dijkstra(graph)
@assert p1_ans == 685
println("p1: $p1_ans")

@assert dijkstra(enlarge(test_graph)) == 315

p2_ans = dijkstra(enlarge(graph))
@assert p2_ans == 2995
println("p2: $p2_ans")
