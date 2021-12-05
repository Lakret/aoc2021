defmodule D05 do
  def parse(input) do
    input
    |> String.trim_trailing()
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [from, to] = String.split(line, " -> ", trim: true)
      %{from: parse_point(from), to: parse_point(to)}
    end)
  end

  defp parse_point(point) do
    [x, y] = String.split(point, ",")
    {x, ""} = Integer.parse(x)
    {y, ""} = Integer.parse(y)
    %{x: x, y: y}
  end

  def horizontal_or_vertical?(segment) do
    segment.from.x == segment.to.x || segment.from.y == segment.to.y
  end

  def trace_grid_line(%{from: %{x: x, y: from_y}, to: %{x: x, y: to_y}}) do
    for y <- from_y..to_y, do: %{x: x, y: y}
  end

  def trace_grid_line(%{from: %{x: from_x, y: y}, to: %{x: to_x, y: y}}) do
    for x <- from_x..to_x, do: %{x: x, y: y}
  end

  def trace_grid_line(%{from: %{x: from_x, y: from_y}, to: %{x: to_x, y: to_y}})
      when abs(to_x - from_x) == abs(to_y - from_y) do
    Enum.zip(from_x..to_x, from_y..to_y)
    |> Enum.map(fn {x, y} -> %{x: x, y: y} end)
  end

  def count_overlaps(segments) do
    {_encountered, overlaps} =
      segments
      |> Enum.reduce({MapSet.new(), MapSet.new()}, fn segment, {encountered, overlaps} ->
        points = trace_grid_line(segment) |> MapSet.new()

        overlaps = MapSet.union(overlaps, MapSet.intersection(points, encountered))
        encountered = MapSet.union(encountered, points)
        {encountered, overlaps}
      end)

    overlaps |> MapSet.size()
  end

  def p1(segments) do
    segments
    |> Enum.filter(&horizontal_or_vertical?/1)
    |> count_overlaps()
  end
end

import ExUnit.Assertions

test_input = """
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"""

test_segments = D05.parse(test_input)
assert D05.p1(test_segments) == 5

segments = File.read!("d05_input") |> D05.parse()

assert D05.p1(segments) |> IO.inspect(label: :p1) == 7473

assert D05.trace_grid_line(%{from: %{x: 9, y: 7}, to: %{x: 7, y: 9}}) == [
         %{x: 9, y: 7},
         %{x: 8, y: 8},
         %{x: 7, y: 9}
       ]

assert D05.count_overlaps(test_segments) == 12
assert D05.count_overlaps(segments) |> IO.inspect(label: :p2) == 24164
