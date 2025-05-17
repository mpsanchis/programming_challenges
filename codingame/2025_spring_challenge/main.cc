#include <cassert>
#include <cstdint>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

// TODO Probably need to use some kind of dynamic programming to store the #
// leafs given the board state, which should make it much faster, but then we
// need to be careful about the depth limit??

// TODO Or maybe we should actually be memoizing the hash rather than the next
// positions

// TODO And then it would be possible to compute this by doing (tree(basenode) -
// tree(basenode after max depth))

#define VECTORIZE_INPUT true // Keep this true

const int32_t DIGITS_MASK[9] = {
    100000000, 10000000, 1000000, 100000, 10000, 1000, 100, 10, 1,
};

const int32_t CAPTURING_SUMS[11][4] = {
    {0, 0, 1, 1}, {0, 1, 0, 1}, {0, 1, 1, 0}, {0, 1, 1, 1},
    {1, 0, 0, 1}, {1, 0, 1, 0}, {1, 0, 1, 1}, {1, 1, 0, 0},
    {1, 1, 0, 1}, {1, 1, 1, 0}, {1, 1, 1, 1},
};

// The first number in each vector is the index of the square where the move is
// made The second to last numbers in each vector are the potential neighbors to
// be captured
const vector<vector<int32_t>> CAPTURING_POSSIBILITIES = {
    {0, 1, 3}, //

    {1, 0, 2, 4}, //
    {1, 0, 2},    //
    {1, 0, 4},    //
    {1, 2, 4},    //

    {2, 1, 5}, //

    {3, 0, 4, 6}, //
    {3, 0, 4},    //
    {3, 0, 6},    //
    {3, 4, 6},    //

    {4, 1, 3, 5, 7}, //
    {4, 1, 3, 5},    //
    {4, 1, 3, 7},    //
    {4, 1, 5, 7},    //
    {4, 3, 5, 7},    //
    {4, 1, 3},       //
    {4, 1, 5},       //
    {4, 1, 7},       //
    {4, 3, 5},       //
    {4, 3, 7},       //
    {4, 5, 7},       //

    {5, 2, 4, 8}, //
    {5, 2, 4},    //
    {5, 2, 8},    //
    {5, 4, 8},    //

    {6, 3, 7}, //

    {7, 4, 6, 8}, //
    {7, 4, 6},    //
    {7, 4, 8},    //
    {7, 6, 8},    //

    {8, 5, 7}, //
};

// Keep track of how many captures for each of the 11 combinations in
// CAPTURING_SUMS Count of 1's in each of the CAPTURING_SUMS combinations
int32_t CAPTURE_COUNTS[11] = {2, 2, 2, 3, 2, 2, 3, 2, 3, 3, 4};

// Cache to store computed next positions for each board state
unordered_map<int32_t, vector<int32_t>> next_positions_cache;

int32_t inline get_board_pos(int32_t board, int32_t x, int32_t y) {
  return board / DIGITS_MASK[x * 3 + y] % 10;
}
int32_t inline get_board_pos(int32_t board, int32_t offset) {
  return board / DIGITS_MASK[offset] % 10;
}

void reached_final_position(const int32_t &board, const int32_t &num_reps,
                            int32_t &result, int32_t &num_seen_leaf_nodes) {
  const int32_t mod_mask = (1 << 30) - 1;

  // Handle large num_reps to avoid overflow
  if (num_reps > (1 << 30)) {
    // (board * num_reps) % 2^30 has a cycle of at most 2^30
    int32_t effective_reps = num_reps % (1 << 30);
    int64_t increment =
        (static_cast<int64_t>(board) * effective_reps) & mod_mask;
    result = (result + increment) & mod_mask;
  } else {
    // Regular case
    int64_t increment = (static_cast<int64_t>(board) * num_reps) & mod_mask;
    result = (result + increment) & mod_mask;
  }

  num_seen_leaf_nodes += num_reps;
}

// TODO Optimize this to avoid the vector and the digit extraction
vector<int32_t> calculate_next_captures_v2(const int32_t &position) {
  // This function was the bottleneck in my previous solution, so I
  // am optimizing maximally by unrolling the loops and hardcoding as much
  // as possible, maximizing data reuse.

  // Reserve a sensible possible number of follow-up positions (empirically), to
  // minimize vector resizing
  vector<int32_t> next_positions;
  next_positions.reserve(8);

#ifdef VECTORIZE_INPUT
  // There will be a lot of reuse, so worth creating
  int32_t position_as_vector[9];
  int32_t current_position = position;
  for (int32_t i = 8; i >= 0; --i) {
    position_as_vector[i] = current_position % 10;
    current_position /= 10;
  }
#endif

  vector<bool> should_place_die_with_value_1(9, true);
  for (const vector<int32_t> &capturing_possibility : CAPTURING_POSSIBILITIES) {
    const int32_t digit = capturing_possibility[0];
#ifdef VECTORIZE_INPUT
    if (position_as_vector[digit] != 0) {
#else
    if (get_board_pos(position, digit) != 0) {
#endif
      // This is not a valid position to place a die
      // This optimizes the check further below
      should_place_die_with_value_1[digit] = false;
      continue;
    }

    // Only search this possibility if ALL neighboring dice are >0
    bool abort_search = false;
    int32_t capture_sum = 0;
    for (uint32_t k = 1; k < capturing_possibility.size(); k++) {
      if (position_as_vector[capturing_possibility[k]] > 0) {
#ifdef VECTORIZE_INPUT
        capture_sum += position_as_vector[capturing_possibility[k]];
#else
        capture_sum += get_board_pos(position, capturing_possibility[k]);
#endif
      } else {
        abort_search = true;
        break;
      }
    }

    if (!abort_search && capture_sum > 0 && capture_sum <= 6) {
      // There is at least one valid capture for this digit,
      // so don't place a die with value 1 for this digit
      should_place_die_with_value_1[digit] = false;

      // Construct the next_position off of the current one
      int32_t next_position = position;
      // Add the die to the position
      next_position += DIGITS_MASK[digit] * capture_sum;
      // Remove the dice from the neighbors
      for (uint32_t k = 1; k < capturing_possibility.size(); k++) {
        next_position -= DIGITS_MASK[capturing_possibility[k]] *
#ifdef VECTORIZE_INPUT
                         position_as_vector[capturing_possibility[k]];
#else
                         get_board_pos(position, capturing_possibility[k]);
#endif
        ;
      }

      next_positions.push_back(next_position);
    }
  }

  for (int32_t digit = 0; digit < 9; digit++) {
    if (should_place_die_with_value_1[digit]) {
      next_positions.push_back(position + DIGITS_MASK[digit]);
    }
  }

  return next_positions;
}

vector<int32_t> calculate_next_captures(const int32_t &position) {
  vector<int32_t> next_positions;

  // Look for available captures
  for (int32_t i = 0; i < 3; i++) {
    for (int32_t j = 0; j < 3; j++) {
      if (get_board_pos(position, i, j) == 0) {
        bool have_captured = false;

        // Look for available captures
        int32_t neighborhood[4] = {
            i - 1 < 0 ? -1 : get_board_pos(position, i - 1, j),
            i + 1 > 2 ? -1 : get_board_pos(position, i + 1, j),
            j - 1 < 0 ? -1 : get_board_pos(position, i, j - 1),
            j + 1 > 2 ? -1 : get_board_pos(position, i, j + 1),
        };

        int32_t combination_ix = 0;
        for (const int32_t (&combination)[4] : CAPTURING_SUMS) {
          int32_t capture_sum = 0;
          int32_t num_dice_to_capture = 0;
          for (int32_t k = 0; k < 4; k++) {
            // A -1 means out of bounds, make it 0 for capture calculation
            int32_t neighbor = neighborhood[k] < 0 ? 0 : neighborhood[k];
            // Only count if the combination wants this neighbor AND the
            // neighbor has a die
            num_dice_to_capture += combination[k] && (neighbor > 0);
            // Add to sum if this neighbor should be captured
            capture_sum += combination[k] && (neighbor > 0) ? neighbor : 0;
          }
          if (num_dice_to_capture == CAPTURE_COUNTS[combination_ix] &&
              capture_sum <= 6) {
            int32_t capture_mask = 0;
            if (combination[0]) {
              capture_mask += DIGITS_MASK[(i - 1) * 3 + j] *
                              get_board_pos(position, i - 1, j);
            }
            if (combination[1]) {
              capture_mask += DIGITS_MASK[(i + 1) * 3 + j] *
                              get_board_pos(position, i + 1, j);
            }
            if (combination[2]) {
              capture_mask += DIGITS_MASK[(i * 3) + (j - 1)] *
                              get_board_pos(position, i, j - 1);
            }
            if (combination[3]) {
              capture_mask += DIGITS_MASK[i * 3 + (j + 1)] *
                              get_board_pos(position, i, j + 1);
            }

            // Captures are mandatory
            int32_t board =
                position + DIGITS_MASK[i * 3 + j] * capture_sum - capture_mask;
            next_positions.push_back(board);
            have_captured = true;
          }
          combination_ix++;
        }

        // Register that you can also just add a die, unless a capture is
        // possible at this location
        if (!have_captured) {
          int32_t board = position + DIGITS_MASK[i * 3 + j];
          next_positions.push_back(board);
        }
      }
    }
  }

  return next_positions;
}

void assert_functional_equivalence_v2(int32_t position,
                                      vector<int32_t> next_boards) {
  vector<int32_t> next_boards_v1 = calculate_next_captures(position);

  assert(next_boards.size() == next_boards_v1.size());

  sort(next_boards.begin(), next_boards.end());
  sort(next_boards_v1.begin(), next_boards_v1.end());

  // for (size_t i = 0; i < next_boards.size(); i++) {
  //   if (next_boards[i] == next_boards_v1[i]) {
  //     cerr << "\033[32mOK\033[0m" << "\t|\t"; // Green text for match
  //   } else {
  //     cerr << "\033[31mNOK\033[0m" << "\t|\t"; // Red text for mismatch
  //   }
  //   cerr << "next_boards[" << i << "] = " << next_boards[i] << "\t|\t";
  //   cerr << "next_boards_v1[" << i << "] = " << next_boards_v1[i] << endl;
  // }
  assert(next_boards == next_boards_v1);
  // cerr << "\033[32mOK\033[0m" << endl; // Green text for match
}

void append_all_legal_moves(unordered_map<int32_t, int32_t> &positions_to_check,
                            int32_t &result, const int32_t &current_depth,
                            const int32_t &max_depth,
                            int32_t &num_seen_leaf_nodes) {
  unordered_map<int32_t, int32_t> board_counts_for_next_depth;
  // int32_t left_to_check = positions_to_check.size();

  for (auto &position : positions_to_check) {
    // left_to_check--;
    // cerr << "current depth: " << current_depth << " | current hash: " <<
    // result
    //      << " | # positions left to process: " << left_to_check << " \r"
    //      << flush;

    if (current_depth >= max_depth) {
      // This position is final due to reaching max depth
      reached_final_position(position.first, position.second, result,
                             num_seen_leaf_nodes);
      continue;
    }

    vector<int32_t> next_boards;
    if (next_positions_cache.find(position.first) !=
        next_positions_cache.end()) {
      // Use cached next positions
      next_boards = next_positions_cache[position.first];
    } else {
      // Calculate next positions

      next_boards = calculate_next_captures_v2(position.first);

      // TODO Clean this up to remove the redundancy after validating the
      // tests
      // For testing only
      // assert_functional_equivalence_v2(position.first, next_boards);

      // next_boards = calculate_next_captures(position.first);

      if (!next_boards.empty()) {
        // Store the computed next positions for this board state
        next_positions_cache[position.first] = next_boards;
      } else {
        // Terminal state (no more moves possible)
        reached_final_position(position.first, position.second, result,
                               num_seen_leaf_nodes);
        continue;
      }
    }

    for (const auto &next_board : next_boards) {
      if (board_counts_for_next_depth.find(next_board) ==
          board_counts_for_next_depth.end()) {
        // This position has not been seen before
        board_counts_for_next_depth[next_board] = position.second;
      } else {
        board_counts_for_next_depth[next_board] += position.second;
      }
    }
  }

  positions_to_check = board_counts_for_next_depth;
}

int32_t main() {
  next_positions_cache.reserve(1 << 18); // needed for the longer inputs

  int32_t max_depth;
  cin >> max_depth;
  cin.ignore();

  int32_t board = 0;
  int32_t digit = 100000000;
  for (int32_t i = 0; i < 3; i++) {
    for (int32_t j = 0; j < 3; j++) {
      int32_t value;
      cin >> value;
      cin.ignore();

      board += value * digit;
      digit /= 10;
    }
  }

  // cerr << "Max depth: " << max_depth << endl;
  // cerr << "Initial board: " << board << endl;

  unordered_map<int32_t, int32_t> positions_to_check;
  positions_to_check[board] = 1;

  int32_t result = 0;
  int32_t num_seen_leaf_nodes = 0;
  int32_t current_depth = 0;
  // Process all positions at each depth level
  while (!positions_to_check.empty()) {
    // cerr << "depth: " << current_depth << "/" << max_depth
    //      << " | seen_leaf_nodes: " << num_seen_leaf_nodes << "\r" << flush;
    append_all_legal_moves(positions_to_check, result, current_depth, max_depth,
                           num_seen_leaf_nodes);
    current_depth++;
  }

  // cerr << endl;
  // cerr << "Number of leaf nodes seen: " << num_seen_leaf_nodes << endl;
  // cerr << "Final hash: " << result << endl;

  cout << result << endl;
}