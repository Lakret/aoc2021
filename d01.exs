defmodule D01 do
  def p1(report, elems \\ 1) do
    {_, increases} =
      Enum.chunk_every(report, elems, 1, :discard)
      |> Enum.map(&Enum.sum/1)
      |> Enum.reduce({nil, 0}, fn
        value, {nil, increases} -> {value, increases}
        value, {prev_value, increases} when prev_value < value -> {value, increases + 1}
        value, {_prev_value, increases} -> {value, increases}
      end)

    increases
  end
end

import ExUnit.Assertions

test_report = [
  199,
  200,
  208,
  210,
  200,
  207,
  240,
  269,
  260,
  263
]

assert D01.p1(test_report) == 7

input =
  File.read!("d01_input")
  |> String.split("\n", trim: true)
  |> Enum.map(fn v ->
    {v, ""} = Integer.parse(v)
    v
  end)

assert D01.p1(input) |> IO.inspect(label: :p1) == 1298

assert D01.p1(test_report, 3) == 5
assert D01.p1(input, 3) |> IO.inspect(label: :p2) == 1248
