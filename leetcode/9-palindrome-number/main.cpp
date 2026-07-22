#include <iostream>
#include <vector>
using namespace std;

#ifdef DEBUG
  #define LOG(msg) std::cout << msg << "\n"
#else
  #define LOG(msg)
#endif


class Solution {
  private:
    vector<int> int2digits(int x) {
      vector<int> digits = {};
      while (x > 9) {
        int last_digit = x % 10;
        x /= 10;
        digits.push_back(last_digit);
      }
      // x < 10: add the last digit
      digits.push_back(x);
      return digits;
    }

  public:
    bool isPalindrome(int x) {
      if (x < 0) {
        return false;
      }
      auto digits = int2digits(x);
      int i_first = 0;
      int i_last = digits.size() - 1;
      while (i_first < i_last) {
        LOG("(first, last) = (" << i_first << "," << i_last << ")");
        if (digits[i_first] != digits[i_last]) {
          return false;
        }
        i_first += 1;
        i_last -= 1;
      }
      return true;
    }
};

int main() {
  auto solver = new Solution();
  int input = 121;
  bool output = solver->isPalindrome(input);
  printf("%i %s a palindrome\n", input, output ? "is" : "is not");

  //
  input = -121;
  output = solver->isPalindrome(input);
  printf("%i %s a palindrome\n", input, output ? "is" : "is not");

  //
  input = 10;
  output = solver->isPalindrome(input);
  printf("%i %s a palindrome\n", input, output ? "is" : "is not");
}
