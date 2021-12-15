using Pipe
using DataStructures

parse_input(input) =
    @pipe input |> chomp |> split(_, "\n") |> map(collect, _) |> hcat(_...) |> permutedims |> parse.(Int32, _)

function dijkstra(graph::Matrix)
    target = maximum(keys(graph))
    current = CartesianIndex(1, 1)
    unvisited = Set(keys(graph))
    distances = Dict(current => graph[current])
    didx = map(CartesianIndex, [(-1, 0), (0, -1), (1, 0), (0, 1)])

    while true
        delete!(unvisited, current)

        for idx in didx
            considered = current + idx

            if considered ∈ keys(graph) && considered != current && considered ∈ unvisited
                new_distance = distances[current] + graph[considered]

                if considered == target
                    return new_distance - distances[CartesianIndex(1, 1)]
                else
                    if considered ∈ keys(distances)
                        distances[considered] = min(distances[considered], new_distance)
                    else
                        distances[considered] = new_distance
                    end

                end
            end
        end

        unvisited_distances = copy(distances)
        for k in keys(unvisited_distances)
            if k ∉ unvisited
                delete!(unvisited_distances, k)
            end
        end

        current = argmin(unvisited_distances)
    end
end

function enlarge(graph::Matrix)
    graph
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

input = read("d15_input", String)
graph = parse_input(input)
@assert dijkstra(graph) == 685

# TODO: p2