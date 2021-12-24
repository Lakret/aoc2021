using Pipe
using DataStructures

# moves:
# (:to_corridor, room_id, corridor_cell_id)
# (:to_room, room_id, corridor_cell_id)
#
# (:to_room, :from_room, room_id1, room_id2)
# is it just (:to_corridor, room_id1, _closes_corridor_cell_id)
# + (:to_room, _cell_id, room_id2)?

target = [:A :A; :B :B; :C :C; :D :D]

# #12.3.4.5.67# - corridor_cell_id
# ###1#2#3#4### - room_id

# (room_id, corridor_cell_id)
steps_to_goal_in_corridor = [
    1 0 0 2 4 6 7
    3 2 0 0 2 4 5
    5 4 2 0 0 2 3
    7 6 4 2 0 0 1
]

function do_move(state, move)
    room_id, corridor_cell_id = move[2], move[3]

    if move[1] == :to_corridor
        room_pos_id = ismissing(state["rooms"][room_id, 1]) ? 2 : 1

        # steps to take to exit the room and get to the next allowed location
        # in the direction of `corridor_cell_id`
        get_out_steps = room_pos_id == 1 ? 2 : 3
        steps = get_out_steps + steps_to_goal_in_corridor[room_id, corridor_cell_id]

        atype = state["rooms"][room_id, room_pos_id]
        cost = steps * atype_to_step_cost(atype)

        new_state = deepcopy(state)
        new_state["rooms"][room_id, room_pos_id] = missing
        new_state["corridor"][corridor_cell_id] = atype

        (cost, new_state)
    elseif move[1] == :to_room
        room_pos_id = ismissing(state["rooms"][room_id, 2]) ? 2 : 1

        get_in_steps = room_pos_id == 1 ? 2 : 3
        steps = get_in_steps + steps_to_goal_in_corridor[room_id, corridor_cell_id]

        atype = state["corridor"][corridor_cell_id]
        cost = steps * atype_to_step_cost(atype)

        new_state = deepcopy(state)
        new_state["rooms"][room_id, room_pos_id] = atype
        new_state["corridor"][corridor_cell_id] = missing

        (cost, new_state)
    else
        @assert false
    end
end

function run(state, moves)
    total_cost = 0

    for move in moves
        (cost, state) = do_move(state, move)
        total_cost += cost
    end

    (total_cost, state)
end

function atype_to_step_cost(atype)
    if atype == :A
        1
    elseif atype == :B
        10
    elseif atype == :C
        100
    elseif atype == :D
        1000
    end
end


# rooms matrix:
# - row - room from left to right
# - column - position in the room, from corridor to the back of the room
#
# corridor: atype and cell_id tuple;
# cell ids count only cells in which it is possible to stop, left to right
test_input = Dict(
    "rooms" => convert(Matrix{Union{Symbol,Missing}}, [:B :A; :C :D; :B :C; :D :A]),
    "corridor" => convert(Vector{Union{Symbol,Missing}}, repeat([missing], 7))
)


test_moves = [
    # B moves out
    (:to_corridor, 3, 3)
    # C moves in
    (:to_corridor, 2, 4)
    (:to_room, 3, 4)
    # D moves out
    (:to_corridor, 2, 4)
    # B moves in
    (:to_room, 2, 3)
    # another B move in
    (:to_corridor, 1, 3)
    (:to_room, 2, 3)
    # D and A evict from 4
    (:to_corridor, 4, 5)
    (:to_corridor, 4, 6)
    # Ds move to thier room
    (:to_room, 4, 5)
    (:to_room, 4, 4)
    # A moves in
    (:to_room, 1, 6)
]

(test_cost, test_final_state) = run(test_input, test_moves)
@assert test_cost == 12521
@assert test_final_state["rooms"] == target
@assert all(ismissing, test_final_state["corridor"])
