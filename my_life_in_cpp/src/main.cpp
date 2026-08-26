#include "./Situation.cpp"
#include <cmath>
#include <iostream>
/**
 * Global Constants
 */

constexpr int calc_people_can_involve(const Situation *situation) {

  if (situation == nullptr) {
    return 0;
  }

  if (situation->type() == SituationType::Love) {

    /* It's gonna default to 2 for now because I believe that true love can only
     * be between two people */
    return 2;
  } else {

    return INFINITY;
  }
}

/**
 * Constants
 */
const Situation my_situation = Situation();
constexpr int MAX_PEOPLE_CAN_INVOLVE = calc_people_can_involve(&my_situation);

/**
 * Returns a bool value of wheather what's in a certain situation is considered
 * love or not
 */
bool is_love(Situation situation) { return false; }

/**
 * Main
 */
int main(int argc, char *argv[]) {
  std::cout << MAX_PEOPLE_CAN_INVOLVE;
  return 0;
}
