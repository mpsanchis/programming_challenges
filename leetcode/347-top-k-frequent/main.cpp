// Compile with:
// clang++ -std=c++20 main.cpp

#include <cassert>
#include <map>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <stdexcept>
#include <iostream>

using namespace std;

class Solution {
  private:
      bool verbose;
      void logln(string msg) {
            if (this->verbose) {
                  cout << msg << "\n";
            }
      }
      void log(string msg) {
            if (this->verbose) {
                  cout << msg;
            }
      }
      map<int, int> get_elem_2_cnt(vector<int>& nums) {
          map<int, int> elem_2_cnt;
          for (int i=0; i<nums.size(); i++) {
            elem_2_cnt[nums[i]]++;
          }
          return elem_2_cnt;
      }
      int find_position(vector<int> &v, int i) {
            auto pos_replace = -1; 
            auto pos_replace_it = std::find(v.begin(), v.end(), i);
            if (pos_replace_it == v.end()) {
                  cout << "Item: " << i << "\n";
                  cout << "Vector:\n";
                  for (int k : v) {
                        cout << k << ',';
                  }
                  cout << "\n";
                  throw std::logic_error("Could not find element with lowest frequency in top_k. Algorithm forgot to add it");
            }
            return pos_replace_it - v.begin();
      }
      int get_elem_min_freq(vector<int>& top_k_elem, map<int,int>& elem_2_freq) {
            int elem_min_freq = top_k_elem[0];
            int min_freq = elem_2_freq[elem_min_freq];
            
            for (int i=1; i<top_k_elem.size(); i++) {
                  auto elem = top_k_elem[i];
                  auto freq = elem_2_freq[elem];
                  if (freq < min_freq) {
                        min_freq = freq;
                        elem_min_freq = elem;
                  }
            }
            return elem_min_freq;
      }
  public:
      Solution(bool verbose) {
            this->verbose = verbose;
      }
      vector<int> topKFrequent(vector<int>& nums, int k) {
            auto elem_2_freq = get_elem_2_cnt(nums);
            // Store top k frequent items in an array
            // and the current element with lowest frequency
            vector<int> top_k_elem;
            std::optional<int> elem_lowest_freq;
            // Iterate map
            for (pair<int, int> elem_2_cnt_entry : elem_2_freq) {
                  auto elem = elem_2_cnt_entry.first;
                  auto freq = elem_2_cnt_entry.second;
                  this->log("elem: ");
                  this->log(to_string(elem));
                  this->log(" - freq: ");
                  this->log(to_string(freq));
                  this->log(" - lowest freq yet: ");
                  this->logln(to_string(elem_lowest_freq.value_or(-1)));

                  // Frequency of current 'elem' is higher than the minimum OR not yet min num of elements
                  // First iteration: no value yet, just add to vector
                  if (!elem_lowest_freq.has_value()) {
                        this->logln("no lowest elem yet.. Adding elem");
                        top_k_elem.push_back(elem);
                        elem_lowest_freq = optional(elem);
                        continue;
                  }
                  int lowest_freq = elem_2_freq[elem_lowest_freq.value()];
                  if (freq > lowest_freq || top_k_elem.size() < k) {
                        // Iterations 1..(k-1): not enough elements yet: just add to vector
                        if (top_k_elem.size() < k) {
                              this->logln("top_k not found yet.. Adding elem");
                              top_k_elem.push_back(elem);
                        }
                        // Iterations k..n: add element only if its frequency is higher than current min
                        else {
                              this->log("finding which element to replace: ");
                              this->logln(to_string(elem_lowest_freq.value()));
                              auto pos_replace = find_position(top_k_elem, elem_lowest_freq.value());
                              top_k_elem[pos_replace] = elem;
                        }
                        elem_lowest_freq = optional(get_elem_min_freq(top_k_elem, elem_2_freq));
                  } else {
                        this->logln("freq too low");
                  }
            }
            return top_k_elem;
    }
};

template<typename T>
void printLnVector(const std::vector<T>& v) {
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i) std::cout << ", ";
        std::cout << v[i];
    }
    std::cout << "]" << "\n";
}

int main() {
      auto solver = new Solution(true);
      // Input: nums = [1,1,1,2,2,3], k = 2
      // Expected: [1,2]
      vector<int> nums = { 1, 1, 1, 2, 2, 3 };
      int k = 2;
      auto solution = solver->topKFrequent(nums, k);
      printLnVector(solution);
      cout << "\n\n";

      // Input: nums = [1], k = 1
      // Expected: [1]
      nums = {1};
      k = 1;
      solution = solver->topKFrequent(nums, k);
      printLnVector(solution);
      cout << "\n\n";

      // Input: nums = [1,2,1,2,1,2,3,1,3,2], k = 2
      // Expected: [1,2]
      nums = {1,2,1,2,1,2,3,1,3,2 };
      k = 2;
      solution = solver->topKFrequent(nums, k);
      printLnVector(solution);
      cout << "\n\n";

      // Input: nums = [3,0,1,0], k = 1
      // Expected: [0]
      nums = {3,0,1,0};
      k = 1;
      solution = solver->topKFrequent(nums, k);
      printLnVector(solution);
      cout << "\n\n";
}
