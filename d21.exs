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

  def p2(player1_pos, player2_pos) do
    all_possible_roll_sums = for r1 <- 1..3, r2 <- 1..3, r3 <- 1..3, do: r1 + r2 + r3

    steps_outcomes_count =
      all_possible_roll_sums
      |> Enum.group_by(fn x -> x end)
      |> Enum.map(fn {k, v} -> {k, length(v)} end)
      |> Enum.into(%{})

    state = %{
      player: :player1,
      # {score, pos} => outcomes_count
      player1: %{
        {0, player1_pos} => 0
      },
      player2: %{
        {0, player2_pos} => 0
      },
      wins: %{
        player1: 0,
        player2: 0
      }
    }

    move_till_win_nd(state, steps_outcomes_count)
  end

  @doc false
  def move_till_win_nd(state, steps_outcomes_count) do
    state = move_nd(state, steps_outcomes_count) |> IO.inspect(label: :state)

    if map_size(state[state.player]) == 0 do
      state
    else
      move_till_win_nd(state, steps_outcomes_count)
    end
  end

  @doc false
  def move_nd(state, steps_outcomes_count) do
    active_player_states =
      Enum.map(
        steps_outcomes_count,
        fn {steps, outcomes} ->
          Enum.reduce(
            state[state.player],
            %{},
            fn {{score, pos}, prev_outcomes}, player_state ->
              new_pos = rem(pos + steps - 1, 10) + 1
              new_score = score + new_pos
              new_outcomes = prev_outcomes + outcomes

              Map.update(
                player_state,
                {new_score, new_pos},
                new_outcomes,
                fn another_branch_outcomes -> new_outcomes + another_branch_outcomes end
              )
            end
          )
        end
      )

    active_player_states =
      Enum.reduce(active_player_states, %{}, fn states_map, merged_state ->
        Map.merge(merged_state, states_map, fn _, v1, v2 -> v1 + v2 end)
      end)

    {active_player_states, wins} =
      Enum.reduce(
        active_player_states,
        {%{}, 0},
        fn
          {{score, _pos}, outcomes}, {states, wins} when score >= 21 ->
            {states, wins + outcomes}

          {score_and_pos, outcomes}, {states, wins} ->
            {Map.put(states, score_and_pos, outcomes), wins}
        end
      )

    %{
      state
      | state.player => active_player_states,
        wins: %{state.wins | state.player => wins},
        player: another_player(state.player)
    }
  end

  defp another_player(:player1), do: :player2
  defp another_player(:player2), do: :player1
end

import ExUnit.Assertions

assert D21.p1(4, 8) == 739_785
assert D21.p1(8, 3) |> IO.inspect(label: :p1) == 412_344
# 1 1
#   2
#   3
# 2 1
#   2
#   3
# 3 1
#   2
#   3
# 1, 2, 3
# 2, 3, 1
# 3, 2, 1

# 27 variants for each turn
# 1, 1, 1 = 3
# 1, 1, 2 = 4
# 1, 1, 3 = 5
# 1, 2, 1 = 4
# 1, 2, 2
# 1, 2, 3
# ...
# 3, 2, 3
# 3, 3, 1
# 3, 3, 2
# 3, 3, 3

all_possible_roll_sums = for r1 <- 1..3, r2 <- 1..3, r3 <- 1..3, do: r1 + r2 + r3

roll_sum_outcomes =
  all_possible_roll_sums
  |> Enum.group_by(fn x -> x end)
  |> Enum.map(fn {k, v} -> {k, length(v)} end)
  |> Enum.into(%{})

Map.keys(roll_sum_outcomes) |> Enum.map(fn steps -> rem(4 + steps - 1, 10) + 1 end)
