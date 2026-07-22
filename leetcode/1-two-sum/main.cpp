#include <cstdio>
#include <vector>
#include <map>

#ifdef DEBUG
  #define LOG(msg)   std::cout << msg
  #define LOGLN(msg) std::cout << msg << "\n"
#else
  #define LOG(msg)
  #define LOGLN(msg)
#endif


using namespace std;

class Solution {
  public:
    vector<int> twoSum(vector<int>& nums, int target) {
        map<int,int> val_2_pos = {};
        for (int pos=0; pos < nums.size(); pos++) {
            auto val = nums[pos];
            auto val_complement = target - val;

            if (val_2_pos.contains(val_complement)) {
                auto pos_complement = val_2_pos.at(val_complement);
                return { pos, pos_complement };
            }
            val_2_pos.insert({ val, pos });
        }
        return { -1, -1 };
    }
};

int main() {
    auto solver = new Solution();
    vector<int> nums = {2,7,11,15};
    auto sol1 = solver->twoSum(nums, 9);
    printf("output: [%i, %i]\n", sol1[0], sol1[1]);

    //
    nums = {3,2,4};
    sol1 = solver->twoSum(nums, 6);
    printf("output: [%i, %i]\n", sol1[0], sol1[1]);

    //
    nums = {3,3};
    sol1 = solver->twoSum(nums, 6);
    printf("output: [%i, %i]\n", sol1[0], sol1[1]);
}
