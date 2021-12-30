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

  @winning_score 21

  @all_rolls for r1 <- 1..3,
                 r2 <- 1..3,
                 r3 <- 1..3,
                 do: r1 + r2 + r3

  @roll_counts Enum.group_by(@all_rolls, fn x -> x end)
               |> Enum.map(fn {k, v} -> {k, length(v)} end)
               |> Enum.into(%{})

  def move_steps(pos, steps)

  # this saves ~30 seconds, by moving computation of the position shifting logic to compile-time
  for pos <- 1..10,
      steps <- 3..9 do
    def move_steps(unquote(pos), unquote(steps)), do: unquote(rem(pos + steps - 1, 10) + 1)
  end

  def p2(p1_pos, p2_pos) do
    {p1_wins, p2_wins} = p2(p1_pos, p2_pos, :player1, 0, 0, 0, 0)

    {max(p1_wins, p2_wins), {p1_wins, p2_wins}}
  end

  defp p2(p1_pos, p2_pos, :player1, p1_score, p2_score, p1_wins, p2_wins) do
    {new_p1_wins, new_p2_wins} =
      Enum.reduce(@roll_counts, {p1_wins, p2_wins}, fn {steps, times}, {p1_wins, p2_wins} ->
        new_pos = move_steps(p1_pos, steps)
        new_score = p1_score + new_pos

        if new_score >= @winning_score do
          {p1_wins + times, p2_wins}
        else
          {new_p1_wins, new_p2_wins} =
            p2(new_pos, p2_pos, :player2, new_score, p2_score, p1_wins, p2_wins)

          update_wins_a_number_of_times(p1_wins, p2_wins, new_p1_wins, new_p2_wins, times)
        end
      end)

    {new_p1_wins, new_p2_wins}
  end

  # this is very copy-pastey, but I found it both easier to read & executing faster than using maps for this
  defp p2(p1_pos, p2_pos, :player2, p1_score, p2_score, p1_wins, p2_wins) do
    {new_p1_wins, new_p2_wins} =
      Enum.reduce(@roll_counts, {p1_wins, p2_wins}, fn {steps, times}, {p1_wins, p2_wins} ->
        new_pos = move_steps(p2_pos, steps)
        new_score = p2_score + new_pos

        if new_score >= @winning_score do
          {p1_wins, p2_wins + times}
        else
          {new_p1_wins, new_p2_wins} =
            p2(p1_pos, new_pos, :player1, p1_score, new_score, p1_wins, p2_wins)

          update_wins_a_number_of_times(p1_wins, p2_wins, new_p1_wins, new_p2_wins, times)
        end
      end)

    {new_p1_wins, new_p2_wins}
  end

  defp update_wins_a_number_of_times(p1_wins, p2_wins, new_p1_wins, new_p2_wins, times) do
    p1_wins = (new_p1_wins - p1_wins) * times + p1_wins
    p2_wins = (new_p2_wins - p2_wins) * times + p2_wins
    {p1_wins, p2_wins}
  end

  defp another_player(:player1), do: :player2
  defp another_player(:player2), do: :player1
end

import ExUnit.Assertions

assert D21.p1(4, 8) == 739_785
assert D21.p1(8, 3) |> IO.inspect(label: :p1) == 412_344

# assert D21.p2(4, 8) == {444_356_092_776_315, {444_356_092_776_315, 341_960_390_180_808}}

{p2_ans, {p1_wins, p2_wins}} = D21.p2(8, 3)
assert p2_wins == 143_154_512_703_677
assert p2_ans == p1_wins
assert p2_ans |> IO.inspect(label: :p2) == 214_924_284_932_572
