defmodule D21 do
  def p1(player1_pos, player2_pos) do
    {winner, state} = play(player1_pos, player2_pos)

    state.scores[another_player(winner)] * state.rolls
  end

  def play(player1_pos, player2_pos) do
    state = %{
      player: :player1,
      positions: %{player1: player1_pos, player2: player2_pos},
      scores: %{player1: 0, player2: 0},
      last_roll: 0,
      rolls: 0
    }

    move_till_win(state)
  end

  @doc false
  def move_till_win(state) do
    state = move(state)

    case Enum.find(state.scores, fn {_, score} -> score >= 1000 end) do
      {winner, _} -> {winner, state}
      nil -> move_till_win(state)
    end
  end

  @doc false
  def move(state) do
    rolls = Enum.map([1, 2, 3], fn x -> state.last_roll + x end)
    steps = Enum.sum(rolls)

    new_pos = rem(state.positions[state.player] + steps - 1, 10) + 1
    new_score = state.scores[state.player] + new_pos

    %{
      player: another_player(state.player),
      positions: %{state.positions | state.player => new_pos},
      scores: %{state.scores | state.player => new_score},
      last_roll: List.last(rolls),
      rolls: state.rolls + 3
    }
  end

  defp another_player(:player1), do: :player2
  defp another_player(:player2), do: :player1
end

import ExUnit.Assertions

assert D21.p1(4, 8) == 739_785
assert D21.p1(8, 3) |> IO.inspect(label: :p1) == 412_344
