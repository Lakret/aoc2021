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
    map =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {line, y}, map ->
        String.graphemes(line)
        |> Enum.with_index()
        |> Enum.reduce(map, fn {cell, x}, map ->
          case cell do
            ">" -> Map.put(map, {x, y}, :east)
            "v" -> Map.put(map, {x, y}, :south)
            _ -> map
          end
        end)
      end)

    %{
      map: map,
      max_x: map |> Map.keys() |> Enum.map(fn {x, _} -> x end) |> Enum.max(),
      max_y: map |> Map.keys() |> Enum.map(fn {_, y} -> y end) |> Enum.max()
    }
  end

  def show(state) do
    for y <- 0..state.max_y do
      for x <- 0..state.max_x do
        case state.map[{x, y}] do
          nil -> "."
          :east -> ">"
          :south -> "v"
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
    new_map =
      Enum.reduce(0..state.max_x, %{}, fn x, map ->
        Enum.reduce(0..state.max_y, map, fn y, map ->
          case state.map[{x, y}] do
            :east ->
              new_x = rem(x + 1, state.max_x + 1)
              neighbour = state.map[{new_x, y}]

              if is_nil(neighbour) do
                Map.put(map, {new_x, y}, :east)
              else
                Map.put(map, {x, y}, :east)
              end

            :south ->
              Map.put(map, {x, y}, :south)

            nil ->
              map
          end
        end)
      end)

    Map.put(state, :map, new_map)
  end

  defp move_south(state) do
    new_map =
      Enum.reduce(0..state.max_x, %{}, fn x, map ->
        Enum.reduce(0..state.max_y, map, fn y, map ->
          case state.map[{x, y}] do
            :east ->
              Map.put(map, {x, y}, :east)

            :south ->
              new_y = rem(y + 1, state.max_y + 1)
              neighbour = state.map[{x, new_y}]

              if is_nil(neighbour) do
                Map.put(map, {x, new_y}, :south)
              else
                Map.put(map, {x, y}, :south)
              end

            nil ->
              map
          end
        end)
      end)

    Map.put(state, :map, new_map)
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
