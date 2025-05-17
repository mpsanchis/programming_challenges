from typing import Dict, List, Callable, Tuple, Any

StateRaw = int  # Type alias
Score = int     # Type alias
TransformationFunction = Callable[[StateRaw], StateRaw]

class State:
    def __init__(self, canonical_state: StateRaw, transform: TransformationFunction):
        self.canonical_state = canonical_state
        self.transform = transform

max_depth = 1

next_states_cache: Dict[StateRaw, List[State]] = {}

def get_input() -> Tuple[StateRaw, int]:
    """
    Return the initial state and the maximum depth.
    TODO: already implemented in C++
    """
    return 0, 1  # initialState, inputMaxDepth

def make_canonical(sr: StateRaw) -> State:
    """
    Convert a raw state to its canonical form.
    TODO: almost implemented already in C++
    Instead of returning a function, we'd return function ID
    """
    return State(
        canonical_state=0,
        transform=lambda sr: sr
    )

def calculate_next_states_raw(sr: StateRaw) -> List[StateRaw]:
    """
    Calculate the next possible raw states from a given raw state.
    TODO: implemented already in C++
    """
    return []

def calculate_next_states(s: State) -> List[State]:
    """Calculate the next possible states from a given state."""
    # check cache
    if s.canonical_state in next_states_cache:
        return next_states_cache[s.canonical_state]

    next_raw_states: List[StateRaw] = calculate_next_states_raw(s.canonical_state)
    next_states: List[State] = [make_canonical(state) for state in next_raw_states]
    
    # populate next states cache and return
    next_states_cache[s.canonical_state] = next_states
    return next_states

def calculate_score(current_state: State, depth: int) -> Score:
    """Calculate the score for a given state and depth."""
    canonical_state = current_state.canonical_state
    transform = current_state.transform

    # Opt: add a score cache(gs_raw, d)

    if depth == max_depth:
        current_state_raw = transform(canonical_state)
        return current_state_raw  # as Score

    next_states: List[State] = calculate_next_states(current_state)
    next_states_scores: List[Score] = []  # same length as 'nextStates'

    for next_state in next_states:
        next_state_score = calculate_score(next_state, depth + 1)
        next_state_score_transformed = transform(next_state_score)  # as StateRaw, then as Score
        next_states_scores.append(next_state_score_transformed)
    
    return sum(next_states_scores)

def main():
    """Main function to run the algorithm."""
    initial_state, input_max_depth = get_input()
    
    global max_depth
    max_depth = input_max_depth
    
    initial_state_canonical = make_canonical(initial_state)
    initial_state_score_canonical = calculate_score(initial_state_canonical, 0)  # not sure if starts with 0 or 1

    initial_state_score = initial_state_canonical.transform(initial_state_score_canonical)  # as StateRaw, then as Score

    print(f"Calculated score: {initial_state_score}")

if __name__ == "__main__":
    main()
