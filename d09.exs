defmodule D09 do
  defstruct [:heatmap, :max_x, :max_y]

  @deltas [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]

  def p1(%__MODULE__{} = m) do
    low_points(m) |> Enum.map(&Map.get(m.heatmap, &1)) |> Enum.map(&(&1 + 1)) |> Enum.sum()
  end

  def p2(%__MODULE__{} = m) do
    basin_sizes(m) |> Enum.sort(:desc) |> Enum.take(3) |> Enum.product()
  end

  def parse(input) do
    heatmap =
      String.trim_trailing(input)
      |> String.split("\n", trim: true)
      |> Enum.with_index(fn row, y ->
        String.graphemes(row)
        |> Enum.with_index(fn v, x ->
          {v, ""} = Integer.parse(v)
          {{x, y}, v}
        end)
      end)
      |> Enum.concat()
      |> Enum.into(%{})

    max_x = heatmap |> Map.keys() |> Enum.map(fn {x, _} -> x end) |> Enum.max()
    max_y = heatmap |> Map.keys() |> Enum.map(fn {_, y} -> y end) |> Enum.max()

    %__MODULE__{heatmap: heatmap, max_x: max_x, max_y: max_y}
  end

  def neighbours(%__MODULE__{} = m, {x, y}) do
    for {dx, dy} <- @deltas, n = {x + dx, y + dy}, m.heatmap |> Map.has_key?(n), do: n
  end

  def low_point?(%__MODULE__{} = m, x, y) do
    v = m.heatmap[{x, y}]

    # nil is larger than any number
    Enum.map(neighbours(m, {x, y}), &(Map.get(m.heatmap, &1) > v)) |> Enum.all?()
  end

  def low_points(%__MODULE__{} = m) do
    for x <- 0..m.max_x, y <- 0..m.max_y, low_point?(m, x, y), do: {x, y}
  end

  def basin_sizes(%__MODULE__{} = m) do
    low_points(m) |> Enum.map(&basin_size(m, &1))
  end

  defp basin_size(%__MODULE__{} = m, low_point) do
    basin_size(m, [low_point], MapSet.new())
  end

  defp basin_size(%__MODULE__{}, [] = _frontier, explored), do: MapSet.size(explored)

  defp basin_size(%__MODULE__{} = m, frontier, explored) do
    explored = MapSet.union(explored, MapSet.new(frontier))

    frontier =
      Enum.flat_map(frontier, &D09.neighbours(m, &1))
      |> Enum.reject(&MapSet.member?(explored, &1))
      |> Enum.filter(&(m.heatmap[&1] != 9))

    basin_size(m, frontier, explored)
  end
end

import ExUnit.Assertions

test_input = """
2199943210
3987894921
9856789892
8767896789
9899965678
"""

test_m = D09.parse(test_input)
m = File.read!("d09_input") |> D09.parse()

assert D09.p1(test_m) == 15
assert D09.p1(m) |> IO.inspect(label: :p1) == 522

assert D09.p2(test_m) == 1134
assert D09.p2(m) |> IO.inspect(label: :p1) == 916_688
