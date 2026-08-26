#include "../include/raylib-cpp.hpp"
#include "raylib.h"

int main() {
  const int screenWidth = 800;
  const int screenHeight = 600;

  // window initialization
  raylib::Window window(screenWidth, screenHeight, "raylib-cpp OOP Sandbox");
  SetTargetFPS(60);

  while (!window.ShouldClose()) {
    float dt = window.GetFrameTime();

    // Begining of drawing
    BeginDrawing();
    // -------------

    window.ClearBackground(raylib::Color::DarkGray());

    raylib::DrawText("Pure raylib-cpp active!", 10, 10, 20,
                     raylib::Color::DarkGray());

    // end of drawing
    EndDrawing();
    // -------------
  }
  return 0;
}
