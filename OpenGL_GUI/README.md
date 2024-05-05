# Safe OpenGL wrapper for the Rust language. 
- https://github.com/glium/glium

# Programming in Rust | Tsoding Daily
- https://youtu.be/AJIyRE2vZ_0?si=FGasHww-MYvgvLdu
  - https://github.com/tsoding/blerp

# 윈도우에 sdl2설치하기

- mingw설치해야함

- https://danmarkjajang.tistory.com/m/9
- https://80000coding.oopy.io/6cea1c29-a2bc-426f-8fe1-8ae9994a8ae9


- msys2로 설치 진행함
  - https://www.msys2.org/
  - https://gist.github.com/thales17/fb2e4cff60890a51d9dddd4c6e832ad2
```
pacman -Syu
```

- Building Tools
```
pacman -S git mingw-w64-x86_64-toolchain mingw64/mingw-w64-x86_64-SDL2 mingw64/mingw-w64-x86_64-SDL2_mixer mingw64/mingw-w64-x86_64-SDL2_image mingw64/mingw-w64-x86_64-SDL2_ttf mingw64/mingw-w64-x86_64-SDL2_net mingw64/mingw-w64-x86_64-cmake make
```

- main.c
```c
#include <stdio.h>
#include <SDL2/SDL.h>

const int WIDTH = 800, HEIGHT = 600;

int main(int argc, char *argv[]) {
  SDL_Window *window;
  SDL_Renderer *renderer;
  if(SDL_Init(SDL_INIT_EVERYTHING) < 0) {
    printf("SDL_Init failed: %s\n", SDL_GetError());
    return 1;
  }

  window = SDL_CreateWindow("Hello, World!",
                                        SDL_WINDOWPOS_UNDEFINED,
                                        SDL_WINDOWPOS_UNDEFINED,
                                        WIDTH, HEIGHT,
                                        SDL_WINDOW_ALLOW_HIGHDPI);
  if(window == NULL) {
    printf("Could not create window: %s\n", SDL_GetError());
    return 1;
  }
  
  renderer = SDL_CreateRenderer(window, -1, 0);
  SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
  SDL_RenderClear(renderer);

  SDL_RenderPresent(renderer);
  
  SDL_Event event;
  while(1) {
    if(SDL_PollEvent(&event)) {
      if(event.type == SDL_QUIT) {
        break;
      }
    }
  }

  SDL_DestroyWindow(window);

  SDL_Quit();
  return 0;
}
```

```
gcc -o hello -Wall hello.c `sdl2-config --cflags --libs`
```
