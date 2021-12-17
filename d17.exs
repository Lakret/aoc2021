defmodule D17 do
  def shoot(velocity) do
    Stream.unfold({{0, 0}, velocity}, fn {{x, y}, {vel_x, vel_y}} ->
      pos = {x + vel_x, y + vel_y}

      vel_x =
        cond do
          vel_x == 0 -> 0
          vel_x > 0 -> vel_x - 1
          vel_x < 0 -> vel_x + 1
        end

      velocity = {vel_x, vel_y - 1}

      {pos, {pos, velocity}}
    end)
  end

  def simulate(velocity, %{min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y}) do
    Enum.reduce_while(shoot(velocity), min_y, fn {x, y}, max_reached_y ->
      max_reached_y = max(max_reached_y, y)

      cond do
        x > max_x or y < min_y -> {:halt, :miss}
        x >= min_x and x <= max_x and y >= min_y and y <= max_y -> {:halt, {:hit, max_reached_y}}
        true -> {:cont, max_reached_y}
      end
    end)
  end

  def p1(target_area) do
    for vel_x <- 1..target_area.max_x,
        vel_y <- 0..(abs(target_area.min_y) * 2) do
      case simulate({vel_x, vel_y}, target_area) do
        {:hit, max_reached_y} -> max_reached_y
        _ -> nil
      end
    end
    |> Enum.reject(&is_nil/1)
    |> Enum.max()
  end

  def p2(target_area) do
    for vel_x <- 1..target_area.max_x,
        vel_y <- (target_area.min_y * 2)..(abs(target_area.min_y) * 2) do
      case simulate({vel_x, vel_y}, target_area) do
        {:hit, _} -> {vel_x, vel_y}
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
target_area = %{min_x: 117, max_x: 164, min_y: -140, max_y: -89}

assert D17.simulate({7, 2}, test_target_area) == {:hit, 3}
assert D17.simulate({6, 3}, test_target_area) == {:hit, 6}
assert D17.simulate({9, 0}, test_target_area) == {:hit, 0}
assert D17.simulate({17, -4}, test_target_area) == :miss
assert D17.simulate({6, 9}, test_target_area) == {:hit, 45}

assert D17.p1(test_target_area) == 45
assert D17.p1(target_area) |> IO.inspect(label: :p1) == 9730

assert D17.p2(test_target_area) == 112
assert D17.p2(target_area) |> IO.inspect(label: :p2) == 4110
