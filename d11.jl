using Pipe
using Plots

parse_input(input) = @pipe input |> chomp |> split |> map(collect, _) |> getindex.(_, Array(1:10)') |> parse.(Int64, _)


function step(energy::Matrix)
    energy = energy .+ 1

    flashed, prev_flashes, flashes = Set(), -1, 0

    while prev_flashes < flashes
        prev_flashes = flashes

        for idx in CartesianIndices(energy)
            min_row, max_row = max(idx.I[1] - 1, 1), min(idx.I[1] + 1, size(energy)[1])
            min_col, max_col = max(idx.I[2] - 1, 1), min(idx.I[2] + 1, size(energy)[2])
            neighbours = [nidx for nidx in CartesianIndices((min_row:max_row, min_col:max_col)) if nidx != idx]

            if idx ∉ flashed && energy[idx] > 9
                push!(flashed, idx)
                energy[neighbours] .+= 1
            end
        end

        flashes = sum(energy .> 9)
    end

    energy[energy.>9] .= 0

    return energy, flashes
end


function step(energy::Matrix, n_steps::Int; save_gif::Bool = false)
    energy, flashes = copy(energy), 0

    anim = @animate for step_id = 1:n_steps
        (energy, new_flashes) = step(energy)
        flashes += new_flashes

        plot(energy, seriestype = :heatmap, c = :oxy, plot_title = "Step $step_id", grid = false, legend = false)
    end

    if save_gif
        gif(anim, "d11.gif", fps = 15)
    end

    energy, flashes
end


function sync_flash(energy::Matrix)
    energy = copy(energy)
    step_idx = 0
    while true
        (energy, _) = step(energy)
        step_idx += 1

        if all(x -> x == 0, energy)
            return step_idx
        end
    end
end

test_input = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""

test_energy = parse_input(test_input)
@assert step(test_energy, 100)[2] == 1656
@assert sync_flash(test_energy) == 195

energy = parse_input(read("d11_input", String))
p1_ans = step(energy, 100)[2]
@assert p1_ans == 1691
println("p1: $p1_ans")

p2_ans = sync_flash(energy)
@assert p2_ans == 216
println("p2: $p2_ans")

# step(energy, 250; save_gif = true)
