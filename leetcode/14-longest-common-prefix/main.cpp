#include <algorithm>
#include <string>
#include <vector>
#include <iostream>

using namespace std;

class Solution {
private:
  int min3(int i1, int i2, int i3) {
    auto min_1 = min(i1, i2);
    return min(min_1, i3);
  }

  int commonPrefixLen(string& s1, string& s2, int common_prefix_len_min) {
    if (s1.length() == 0 || s2.length() == 0) {
      return -1;
    }
    if (s1.at(0) != s2.at(0)) {
      return -1;
    }

    int i = 0;
    for (; i < this->min3(s1.size(), s2.size(), common_prefix_len_min); ++i) {
      if (s1.at(i) != s2.at(i)) {
        break;
      }
    }
    return i;
  }
public:
    string longestCommonPrefix(vector<string>& strs) {
        auto first = strs[0];
        if (strs.size() == 1) {
          return first;
        }

        int common_prefix_len_min = first.length();
        for (int i=1; i<strs.size(); ++i) {
          int common_prefix_len = this->commonPrefixLen(first, strs[i], common_prefix_len_min);
          if (common_prefix_len < 0) {
            return "";
          }
          if (common_prefix_len < common_prefix_len_min) {
            common_prefix_len_min = common_prefix_len;
          }
        }
        return first.substr(0, common_prefix_len_min);
    }
};

int main() {
  auto solver = new Solution();
  vector<string> strs = {"flower","flow","flight"};
  string sol1 = solver->longestCommonPrefix(strs);
  cout << "longest common prefix: " << sol1 << "\n";

  //
  strs = { "dog","racecar","car" };
  sol1 = solver->longestCommonPrefix(strs);
  cout << "longest common prefix: " << sol1 << "\n";

  //
  strs = { "foobar","foooar","fw" };
  sol1 = solver->longestCommonPrefix(strs);
  cout << "longest common prefix: " << sol1 << "\n";

  //
  strs = { "asdasd123" };
  sol1 = solver->longestCommonPrefix(strs);
  cout << "longest common prefix: " << sol1 << "\n";
}
