#include <iostream>

int main() {
  // \033[1] sets the terminal layout engine to Right-to-Left mode
  // \033[0] resets it back to Left-to-Right mode so your prompt doesn't break
  std::cout << "\033[1]😂🤨 برضو جديد مفيش\033[0]\n";

  return 0;
}
