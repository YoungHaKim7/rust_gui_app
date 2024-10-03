# rust_GUI_app

# Rust GUI 종류(전체적으로 찾아보기)
- https://areweguiyet.com/
- 레딧글 https://www.reddit.com/r/rust/comments/10wy4yw/rust_gui_framework/

<hr>
    
- **[Vortex - OpenCL 호환 가능한 RISC-V 아키텍처 기반의 풀스택 오픈소스 GPGPU](<https://news.hada.io/topic?id=14297&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- RISC-V ISA 확장을 기반으로 GPGPU를 지원하는 오픈 소스 하드웨어 및 소프트웨어 프로젝트  
- 현재는 OpenCL 1.2를 지원하며 FPGA에서 실행됨  
- RISC-V RV32IMAF 와 RV64IMAFD 지원   
- GPU 아키텍처 연구를 가능하게 하는 완전한 오픈 소스 컴파일러, 드라이버 및 런타임 소프트웨어 스택을 갖추고 있으며 높...
  - OpenCL의 현재 상황:
    - AMD와 Intel은 OpenCL에서 ROCm, DPC++ 등 다른 GPGPU 언어로 전환 중인 것으로 보임
    - 그럼에도 불구하고 OpenCL은 최고의 컴퓨팅 API라는 의견도 있음
  - GPU 컴퓨팅의 현 상황이 매우 복잡함:
    - OpenGL, OpenCL: 널리 지원되나 최신 성능은 부족
    - Vulkan, Metal, DirectX: 최신 그래픽 API로 좋은 드라이버 지원과 성능을 가짐
    - CUDA, HIP: 사유 API로 CUDA는 NVIDIA GPU만, HIP은 AMD GPU도 지원
    - SYCL: 이기종 컴퓨팅 프레임워크로 OpenCL/Vulkan 등의 코드 생성 가능
    - WebGPU: 다양한 구현체가 있는 웹 표준
  - 향후 5~10년 내 오픈소스 기반의 프로세서와 소프트웨어 발전에 대한 기대감이 있음

<br>

<hr>

# Rust A Simple GUI Library

https://google.github.io/comprehensive-rust/exercises/day-3/solutions-morning.html

# Rust GUI

- Druid GUI[![deprecated](http://badges.github.io/stability-badges/dist/deprecated.svg)][![crates.io](https://img.shields.io/crates/v/druid.svg)](https://crates.io/crates/druid)![Crates.io](https://img.shields.io/crates/l/druid)![druidDownloads](https://img.shields.io/crates/d/druid.svg)<a href="https://github.com/inebender/druid"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/linebender/druid.svg)

    - A data-first Rust-native UI design toolkit.
    
    - https://github.com/linebender/druid

    - https://linebender.org/druid/

    - https://github.com/YoungHaKim7/druid-example

# Xilem(Druid계보를 이어 받음 역시)
- xilem[![crates.io](https://img.shields.io/crates/v/xilem.svg)](https://crates.io/crates/xilem)![Crates.io](https://img.shields.io/crates/l/xilem)
![wgpuDownloads](https://img.shields.io/crates/d/xilem.svg)<a href="https://github.com/linebender/xilem"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/linebender/xilem.svg)

    - An experimental Rust native UI framework

    - Rust 용 Data-first UI 디자인 툴킷인 Druid 를 만들었던 사람의 제안 글이네요. 꽤 긴글이라 도입부만 옮겨봅니다. 결과물이 나와봐야 알 것 같아요. Raph Levien은 사실 Druid만으로 말하기에는 너무 부족한 사람인데... (GPL) Ghostscript 메인테이너, Advogato 블로그 커뮤니티, Inconsolata 글꼴, Xi 편집기 등으로 예전부터 유명합니다.
      - https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html
      - https://news.hada.io/topic?id=6519

    - https://github.com/linebender/xilem

- Xilem최신글
  - 240119 https://linebender.org/blog/xilem-backend-roadmap/

- Project status
  - The Druid project is being discontinued by the core developer team.

  - New development effort is focused on Xilem, which has a lot of fundamental changes to allow for a wider variety of applications with better performance, but it also heavily inherits from Druid. We see Xilem as the future of Druid.

  - Druid is reasonably usable for some subset of applications and has a significant testing history, which ensures some stability and correctness. However we don't expect any major new features to be added to Druid. As such we don't recommend using Druid for brand new applications. If you insist, then at least make sure your application doesn't require a feature that Druid doesn't have, e.g. accessibility or 3D support.

<hr>

- (egui)[![crates.io](https://img.shields.io/crates/v/egui.svg)](https://crates.io/crates/egui)![Crates.io](https://img.shields.io/crates/l/egui)
![wgpuDownloads](https://img.shields.io/crates/d/egui.svg)<a href="https://github.com/emilk/egui"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![eguiustar](https://img.shields.io/github/stars/emilk/egui.svg)

    - egui: an easy-to-use immediate mode GUI in Rust that runs on both web and native

    - https://github.com/emilk/egui

- iced[![crates.io](https://img.shields.io/crates/v/iced.svg)](https://crates.io/crates/iced)
![Crates.io](https://img.shields.io/crates/l/iced)
![icedDownloads](https://img.shields.io/crates/d/iced.svg)<a href="https://github.com/iced-rs/iced"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![icedstar](https://img.shields.io/github/stars/iced-rs/iced.svg)

    - (iced) A cross-platform GUI library for Rust, inspired by Elm

    - https://github.com/iced-rs/iced

- relm[![crates.io](https://img.shields.io/crates/v/relm.svg)](https://crates.io/crates/relm)
![Crates.io](https://img.shields.io/crates/l/relm)
![icedDownloads](https://img.shields.io/crates/d/relm.svg)<a href="https://github.com/antoyo/relm"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![icedstar](https://img.shields.io/github/stars/antoyo/relm.svg)

    - (relm) Idiomatic, GTK+-based, GUI library, inspired by Elm, written in Rust
    - https://github.com/antoyo/relm
    - https://crates.io/crates/relm
   
- relm4 GUI[![crates.io](https://img.shields.io/crates/v/relm4.svg)](https://crates.io/crates/reml4)![Crates.io](https://img.shields.io/crates/l/egui)![druidDownloads](https://img.shields.io/crates/d/relm4.svg)<a href="https://github.com/Relm4/Relm4"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/Relm4/Relm4.svg)<a href="https://docs.rs/relm4/" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/badge/rust-documentation-blue" alt="Relm4 docs"></a><a href="https://relm4.org/book/stable/" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/badge/rust-book-fc0060" alt="Relm4 book"></a>

    - An idiomatic GUI library inspired by Elm and based on gtk4-rs. Relm4 is a new version of relm that's built from scratch and is compatible with GTK4 and libadwaita. 
    
    - https://github.com/Relm4/Relm4

    - https://relm4.org/

    - https://crates.io/crates/relm4

<hr>

# Window handling library in pure Rust
- winit_GUI[![crates.io](https://img.shields.io/crates/v/winit.svg)](https://crates.io/crates/winit)![Crates.io](https://img.shields.io/crates/l/winit)![winitDownloads](https://img.shields.io/crates/d/winit.svg)<a href="https://github.com/rust-windowing/winit"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![winitdstar](https://img.shields.io/github/stars/rust-windowing/winit.svg)

  - https://github.com/rust-windowing/winit

  - https://docs.rs/winit/latest/winit/

  - https://crates.io/crates/winit




<hr>

# windows-rs
- windows-rs![icedstar](https://img.shields.io/github/stars/microsoft/windows-rs.svg)

    - Rust for Windows
      - https://kennykerr.ca/rust-getting-started/

    - https://github.com/microsoft/windows-rs
    - This repo is the home of the following crates (and other supporting crates):

        - windows-sys - Raw bindings for C-style Windows APIs.
        - windows - Safer bindings including C-style APIs as well as COM and WinRT APIs.
        - windows-core - Type support for the windows crate.
        - windows-implement - The implement macro for the windows crate, for implementing COM interfaces.
        - windows-interface - The interface macro for the windows crate, for declaring COM interfaces.
        - windows-targets - Import libs for Windows.
        - windows-version - Windows version information.
        - windows-metadata - Windows metadata reader.
        - windows-bindgen - Windows metadata compiler library.
        - riddle - Windows metadata compiler tool.

- relm[![crates.io](https://img.shields.io/crates/v/relm.svg)](https://crates.io/crates/relm)
![Crates.io](https://img.shields.io/crates/l/relm)
![icedDownloads](https://img.shields.io/crates/d/relm.svg)<a href="https://github.com/antoyo/relm"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![icedstar](https://img.shields.io/github/stars/antoyo/relm.svg)

    - (relm) Idiomatic, GTK+-based, GUI library, inspired by Elm, written in Rust
    - https://github.com/antoyo/relm
    - https://crates.io/crates/relm
   
- relm4 GUI[![crates.io](https://img.shields.io/crates/v/relm4.svg)](https://crates.io/crates/reml4)![Crates.io](https://img.shields.io/crates/l/egui)![druidDownloads](https://img.shields.io/crates/d/relm4.svg)<a href="https://github.com/Relm4/Relm4"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/Relm4/Relm4.svg)<a href="https://docs.rs/relm4/" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/badge/rust-documentation-blue" alt="Relm4 docs"></a><a href="https://relm4.org/book/stable/" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/badge/rust-book-fc0060" alt="Relm4 book"></a>

    - An idiomatic GUI library inspired by Elm and based on gtk4-rs. Relm4 is a new version of relm that's built from scratch and is compatible with GTK4 and libadwaita. 
    
    - https://github.com/Relm4/Relm4

    - https://relm4.org/

    - https://crates.io/crates/relm4

- fltk-rs[![crates.io](https://img.shields.io/crates/v/fltk.svg)](https://crates.io/crates/fltk)![Crates.io](https://img.shields.io/crates/l/fltk)![druidDownloads](https://img.shields.io/crates/d/fltk.svg)<a href="https://github.com/fltk-rs/fltk-rs"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/fltk-rs/fltk-rs.svg)

    - Rust bindings for the FLTK GUI library.
    
    - https://github.com/fltk-rs/fltk-rs
 
    - https://crates.io/crates/fltk
      - Tutorial Video(FLTK Rust)
        - https://youtube.com/playlist?list=PLHqrrowPLkDu9U-uk60sGM-YWLOJFfLoE

<br>

<hr>

## The Rust UI-Toolkit.<br>

- Masonry-rs[![crates.io](https://img.shields.io/crates/v/masonry.svg)](https://crates.io/crates/masonry)![Crates.io](https://img.shields.io/crates/l/masonry)![druidDownloads](https://img.shields.io/crates/d/masonry.svg)<a href="https://github.com/PoignardAzur/masonry-rs"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/PoignardAzur/masonry-rs.svg)

    - Rust UI design toolkit.

    - This project was originally a fork of Druid that emerged from discussions I had with Raph Levien and Colin Rofls about what it would look like to turn Druid into a foundational library.

    - https://github.com/PoignardAzur/masonry-rs




- Orbtk[![deprecated](http://badges.github.io/stability-badges/dist/deprecated.svg)](http://github.com/badges/stability-badges)[![crates.io](https://img.shields.io/crates/v/orbtk.svg)](https://crates.io/crates/orbtk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![icedDownloads](https://img.shields.io/crates/d/orbtk.svg)<a href="https://github.com/redox-os/orbtk"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![icedstar](https://img.shields.io/github/stars/redox-os/orbtk.svg)

    - The Orbital Widget Toolkit is a cross-platform (G)UI toolkit for building scalable user interfaces with the programming language Rust

    - It is with great sadness that I announce that OrbTk is sunsetting

    - https://github.com/redox-os/orbtk

<br>

<hr>

<br>

