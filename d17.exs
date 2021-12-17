defmodule D17 do
  def shoot(velocity) do
    Stream.unfold({{0, 0}, velocity}, fn {{x, y}, {velocity_x, velocity_y}} ->
      pos = {x + velocity_x, y + velocity_y}

      velocity_x =
        cond do
          velocity_x == 0 -> 0
          velocity_x > 0 -> velocity_x - 1
          velocity_x < 0 -> velocity_x + 1
        end

      velocity = {velocity_x, velocity_y - 1}

      {pos, {pos, velocity}}
    end)
  end

  def simulate(velocity, %{min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y}) do
    Enum.reduce_while(shoot(velocity), min_y, fn {x, y}, max_reached_y ->
      max_reached_y = max(max_reached_y, y)

      cond do
        x > max_x -> {:halt, :overshoot}
        y < min_y and x < min_x -> {:halt, :undershoot}
        y < min_y and x > min_x -> {:halt, :too_high}
        x >= min_x and x <= max_x and y >= min_y and y <= max_y -> {:halt, {:hit, max_reached_y}}
        true -> {:cont, max_reached_y}
      end
    end)
  end

  def optimize(target_area) do
    xs = 1..target_area.max_x |> Enum.to_list()
    ys = 0..300 |> Enum.to_list()

    for x <- xs, y <- ys do
      case simulate({x, y}, target_area) do
        {:hit, max_reached_y} -> {{x, y}, max_reached_y}
        _ -> nil
      end
    end
    |> Enum.reject(&is_nil/1)
    |> Enum.into(%{})
    |> Enum.max_by(fn {k, v} -> v end)
  end

  def p2(target_area) do
    xs = 1..target_area.max_x |> Enum.to_list()
    ys = -300..300 |> Enum.to_list()

    for x <- xs, y <- ys do
      case simulate({x, y}, target_area) do
        {:hit, _} -> {x, y}
        _ -> nil
      end
    end
    |> Enum.reject(&is_nil/1)
    |> Enum.dedup()
    |> length()
  end
end

import ExUnit.Assertions

test_target_area = %{min_x: 20, max_x: 30, min_y: -10, max_y: -5}

D17.shoot({7, 2}) |> Enum.take(8)

assert D17.simulate({7, 2}, test_target_area) == {:hit, 3}
assert D17.simulate({6, 3}, test_target_area) == {:hit, 6}
assert D17.simulate({9, 0}, test_target_area) == {:hit, 0}
assert D17.simulate({17, -4}, test_target_area) == :overshoot
assert D17.simulate({6, 9}, test_target_area) == {:hit, 45}

assert D17.optimize(test_target_area) == {{6, 9}, 45}

target_area = %{min_x: 117, max_x: 164, min_y: -140, max_y: -89}
assert D17.optimize(target_area) == {{15, 139}, 9730}

assert D17.p2(test_target_area) == 112
assert D17.p2(target_area) == 4110
