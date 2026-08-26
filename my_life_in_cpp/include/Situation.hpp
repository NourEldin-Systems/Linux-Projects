#include "./Person.hpp"

enum class SituationType { Love, Friendship };

class Situation {
private:
  Person _people_involved[10];
  SituationType _type;

public:
  Situation(SituationType type) {
    // Here we're gonna initialize the situation
    _type = type;
  }

  SituationType type() { return _type; }
};
