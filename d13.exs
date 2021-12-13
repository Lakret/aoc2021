defmodule D13 do
  def parse(input) do
    [coords, folds] =
      input
      |> String.trim_trailing()
      |> String.split("\n\n", trim: true)

    coords =
      coords
      |> String.split("\n", trim: true)
      |> Enum.map(&String.split(&1, ",", trim: true))
      |> Enum.map(fn [x, y] ->
        {String.to_integer(x), String.to_integer(y)}
      end)

    folds =
      folds
      |> String.split("\n", trim: true)
      |> Enum.map(fn f ->
        [kind, val] =
          f
          |> String.trim_leading("fold along ")
          |> String.split("=", trim: true)

        {kind, String.to_integer(val)}
      end)

    %{coords: coords, folds: folds}
  end

  def do_fold(%{coords: coords, folds: folds}) do
    Enum.reduce(folds, coords, fn f, coords -> do_fold(coords, f) end)
  end

  def do_fold(coords, {"y", val}) do
    Enum.map(coords, fn
      {x, y} when y <= val ->
        {x, y}

      {x, y} when y > val ->
        {x, 2 * val - y}
    end)
    |> Enum.uniq()
  end

  def do_fold(coords, {"x", val}) do
    Enum.map(coords, fn
      {x, y} when x <= val ->
        {x, y}

      {x, y} when x > val ->
        {2 * val - x, y}
    end)
    |> Enum.uniq()
  end

  def p1(puzzle) do
    do_fold(puzzle.coords, puzzle.folds |> Enum.at(0))
    |> length()
  end

  def p2(puzzle) do
    dots = do_fold(puzzle) |> MapSet.new()
    max_x = dots |> Enum.map(fn {x, _} -> x end) |> Enum.max()
    max_y = dots |> Enum.map(fn {_, y} -> y end) |> Enum.max()

    for y <- 0..max_y do
      line =
        for x <- 0..max_x do
          if {x, y} in dots do
            "x"
          else
            " "
          end
        end

      line |> Enum.join()
    end
    |> Enum.join("\n")
  end
end

import ExUnit.Assertions

test_input = """
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"""

test_puzzle = D13.parse(test_input)
assert D13.p1(test_puzzle) == 17
assert D13.do_fold(test_puzzle) |> length() == 16

puzzle = File.read!("d13_input") |> D13.parse()
assert D13.p1(puzzle) == 751
D13.p2(puzzle) |> IO.puts()
