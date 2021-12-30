defmodule D25 do
  def p1(state) do
    [{{state, _}, idx}] =
      Stream.unfold(state, fn state ->
        new_state = D25.move(state)
        {{state, new_state}, new_state}
      end)
      |> Stream.with_index()
      |> Stream.drop_while(fn {{prev_state, state}, _idx} -> prev_state != state end)
      |> Enum.take(1)

    {idx + 1, state}
  end

  def parse(input) do
    {east, south} =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.reduce({MapSet.new(), MapSet.new()}, fn {line, y}, maps ->
        line
        |> String.graphemes()
        |> Enum.with_index()
        |> Enum.reduce(maps, fn {cell, x}, {east, south} ->
          case cell do
            ">" -> {MapSet.put(east, {x, y}), south}
            "v" -> {east, MapSet.put(south, {x, y})}
            _ -> {east, south}
          end
        end)
      end)

    all = MapSet.union(east, south)
    max_x = Stream.map(all, fn {x, _} -> x end) |> Enum.max()
    max_y = Stream.map(all, fn {_, y} -> y end) |> Enum.max()

    %{east: east, south: south, all: all, max_x: max_x, max_y: max_y}
  end

  def show(state) do
    for y <- 0..state.max_y do
      for x <- 0..state.max_x do
        cond do
          {x, y} in state.east -> ">"
          {x, y} in state.south -> "v"
          true -> "."
        end
      end
    end
    |> Enum.intersperse("\n")
    |> IO.puts()
  end

  def move(state) do
    state
    |> move_east()
    |> move_south()
  end

  defp move_east(state) do
    east =
      Enum.reduce(state.east, MapSet.new(), fn {x, y}, east ->
        new_x = rem(x + 1, state.max_x + 1)

        if {new_x, y} in state.all do
          MapSet.put(east, {x, y})
        else
          MapSet.put(east, {new_x, y})
        end
      end)

    %{state | east: east, all: MapSet.union(east, state.south)}
  end

  defp move_south(state) do
    south =
      Enum.reduce(state.south, MapSet.new(), fn {x, y}, south ->
        new_y = rem(y + 1, state.max_y + 1)

        if {x, new_y} in state.all do
          MapSet.put(south, {x, y})
        else
          MapSet.put(south, {x, new_y})
        end
      end)

    %{state | south: south, all: MapSet.union(state.east, south)}
  end
end

import ExUnit.Assertions

test_input = """
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"""

test_state = D25.parse(test_input)

input = File.read!("d25_input") |> String.trim_trailing()
state = D25.parse(input)

{test_answer, _} = D25.p1(test_state)
assert test_answer == 58

{answer, final_state} = D25.p1(state)
assert answer |> IO.inspect(label: :p1) == 424

D25.show(state)
IO.puts("")
IO.puts("Final state:")
D25.show(final_state)
