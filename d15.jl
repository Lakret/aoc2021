using Pipe
using DataStructures

parse_input(input) =
    @pipe input |> chomp |> split(_, "\n") |> map(collect, _) |> hcat(_...) |> permutedims |> parse.(Int32, _)

function dijkstra(graph::Matrix)
    target = maximum(keys(graph))
    current = CartesianIndex(1, 1)
    distances = Dict(current => graph[current])
    didx = map(CartesianIndex, [(-1, 0), (0, -1), (1, 0), (0, 1)])

    unvisited = PriorityQueue()
    for idx in keys(graph)
        unvisited[idx] = Inf
    end

    while true
        for idx in didx
            considered = current + idx

            if considered ∈ keys(graph) && considered != current && considered ∈ keys(unvisited)
                new_distance = distances[current] + graph[considered]

                if considered == target
                    return new_distance - distances[CartesianIndex(1, 1)]
                else
                    if considered ∈ keys(distances)
                        distances[considered] = min(distances[considered], new_distance)
                    else
                        distances[considered] = new_distance
                    end

                    unvisited[considered] = distances[considered]
                end
            end
        end

        current = dequeue!(unvisited)
    end
end


function enlarge(graph::Matrix)
    rows = []
    for drow = 1:5
        row = []
        for dcol = 1:5
            tile = (graph .- 1 .+ (drow - 1) .+ (dcol - 1)) .% 9 .+ 1
            tile[tile.==0] .= 1

            push!(row, tile)
        end

        row = hcat(row...)
        push!(rows, row)
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
@assert dijkstra(test_graph) == 40

input = read("d15_input", String);
graph = parse_input(input)
@assert dijkstra(graph) == 685

@assert dijkstra(enlarge(test_graph)) == 315
@assert dijkstra(enlarge(graph)) == 2995
