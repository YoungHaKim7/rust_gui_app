# Rust FFI

- https://doc.rust-lang.org/nomicon/ffi.html

<hr>

# cxx-qt
- Safe interop between Rust and Qt

  - https://github.com/KDAB/cxx-qt

# The CXX-Qt Book walks through a minimal example step-by-step and documents CXX-Qt's features for the latest release.

- https://kdab.github.io/cxx-qt/book/getting-started/index.html

  - Getting Started - CXX-Qt Documentation
    - 이번에 투자 받았다고 하는곳 c++ 코드 => 러스트 코드로 변환 작업 중 :rust~1:
    - news 출처(Rust Foundation) :
      - https://foundation.rust-lang.org/news/google-contributes-1m-to-rust-foundation-to-support-c-rust-interop-initiative/
      - https://thenewstack.io/google-spends-1-million-to-make-rust-c-interoperable/

# macos QT

https://doc.qt.io/qt-6/macos.html

<hr>

- https://github.com/rust-qt
  - https://wiki.qt.io/Language_Bindings

- QT6 Install
  - https://www.qt.io/download-qt-installer

- Document(QT5)
  - https://doc.qt.io/
  - Qt Graphs
    - https://doc.qt.io/qt-6/qtgraphs-index.html?utm_source=installer&utm_medium=banner&utm_campaign=installer4Qt66

- QT6 tutorial
  - https://doc-snapshots.qt.io/qt6-dev/qt-intro.html

- QTCreator(Fakevim)
  - https://doc.qt.io/qtcreator/creator-editor-fakevim.html


<hr>

# ```export QT_QPA_PLATFORM=eglfs ```
```
./analogclock 
qt.qpa.plugin: Could not find the Qt platform plugin "xcb" in ""
This application failed to start because no Qt platform plugin could be initialized. Reinstalling the application may fix this problem.

Available platform plugins are: eglfs, linuxfb, minimal, minimalegl, offscreen, vnc, wayland-egl, wayland, webgl.

[1]    3168 IOT instruction (core dumped)  ./analogclock

```

# PATH

- bashrc
```
##### QT #####
export SW_ROOT=$HOME/utilities
export QT5_ROOT=$SW_ROOT/qt5

export USER_LIB=$QT5_ROOT/lib
export SYS_LIB=$/lib:/lib64:/usr/lib:/usr/lib64

export LD_LIBRARY_PATH=$USER_LIB:$SYS_LIB
export QT_PLUGIN_PATH=$QT5_ROOT/plugins
############
```

# QT5 DEBUG message

```
export QT_DEBUG_PLUGINS=1
```
- https://forum.qt.io/topic/116299/qt-creator-ubuntu-20-04/4

# QT6 Install

- 병렬로 빨리 설치하기(속이 시원하다)
```
cmake --build . --parallel
```

# QT5 Install
- https://wiki.qt.io/Building_Qt_5_from_Git#Getting_the_source_code
- https://download.qt.io/
- https://softengn.tistory.com/4

- qt-creator
  - https://github.com/qt-creator/qt-creator
    - 설명서
      - https://doc.qt.io/qtcreator/index.html

```
sudo apt-get install libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev libxcb-shape0-dev libxcb-xkb-dev
```

- https://www.linuxfromscratch.org/blfs/view/svn/x/qt5.html


```
./Configure

Qt is now configured for building. Just run 'gmake'.
Once everything is built, you must run 'gmake install'.
Qt will be installed into '/usr/local/Qt-5.15.12'.

```

- 폴더 다른게 세팅해줌
```
./configure --prefix=/home/gy/utilities/qt5 -shared
```

- gmake설치 오래 걸림 ㅜㅜ
- 설치 설명서
  - https://saelly.tistory.com/m/567

```
gmake[4]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc/examples/demos/rssnews'
gmake[3]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc/examples/demos'
gmake[2]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc/examples'
cd doc/src/cmake/ && ( test -e Makefile || /home/gy/utilities/qt_5_15_12/qtbase/bin/qmake -o Makefile /home/gy/utilities/qt_5_15_12/qtdoc/doc/src/cmake/cmake.pro ) && gmake -f Makefile 
gmake[2]: Entering directory '/home/gy/utilities/qt_5_15_12/qtdoc/doc/src/cmake'
gmake[2]: Nothing to be done for 'first'.
gmake[2]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc/doc/src/cmake'
cd doc/ && ( test -e Makefile || /home/gy/utilities/qt_5_15_12/qtbase/bin/qmake -o Makefile /home/gy/utilities/qt_5_15_12/qtdoc/doc/doc.pro ) && gmake -f Makefile 
gmake[2]: Entering directory '/home/gy/utilities/qt_5_15_12/qtdoc/doc'
gmake[2]: Nothing to be done for 'first'.
gmake[2]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc/doc'
gmake[1]: Leaving directory '/home/gy/utilities/qt_5_15_12/qtdoc'
```
- 내 설치 상태 ```./config.status```
```
./config.status
+ cd qtbase
+ /home/gy/utilities/qt_5_15_12/qtbase/configure -top-level -redo
Creating qmake...
.Done.

This is the Qt Open Source Edition.

You have already accepted the terms of the Open Source license.

Running configuration tests...
Done running configuration tests.

Configure summary:

Build type: linux-g++ (x86_64, CPU features: mmx sse sse2)
Compiler: gcc 11.4.0
Configuration: sse2 aesni sse3 ssse3 sse4_1 sse4_2 avx avx2 avx512f avx512bw avx512cd avx512dq avx512er avx512ifma avx512pf avx512vbmi avx512vl compile_examples enable_new_dtags f16c largefile precompile_header rdrnd rdseed shani x86SimdAlways shared shared rpath release c++11 c++14 c++17 c++1z concurrent dbus reduce_exports reduce_relocations stl
Build options:
  Mode ................................... release
  Optimize release build for size ........ no
  Building shared libraries .............. yes
  Using C standard ....................... C11
  Using C++ standard ..................... C++17
  Using ccache ........................... no
  Using new DTAGS ........................ yes
  Relocatable ............................ yes
  Using precompiled headers .............. yes
  Using LTCG ............................. no
  Target compiler supports:
    SSE .................................. SSE2 SSE3 SSSE3 SSE4.1 SSE4.2
    AVX .................................. AVX AVX2
    AVX512 ............................... F ER CD PF DQ BW VL IFMA VBMI
    Other x86 ............................ AES F16C RDRAND SHA
    Intrinsics without -mXXX option ...... yes
  Build parts ............................ libs examples tools
Qt modules and options:
  Qt Concurrent .......................... yes
  Qt D-Bus ............................... yes
  Qt D-Bus directly linked to libdbus .... no
  Qt Gui ................................. yes
  Qt Network ............................. yes
  Qt Sql ................................. yes
  Qt Testlib ............................. yes
  Qt Widgets ............................. yes
  Qt Xml ................................. yes
Support enabled for:
  Using pkg-config ....................... yes
  udev ................................... no
  Using system zlib ...................... yes
  Zstandard support ...................... no
Qt Core:
  DoubleConversion ....................... yes
    Using system DoubleConversion ........ no
  GLib ................................... no
  iconv .................................. no
  ICU .................................... yes
  Built-in copy of the MIME database ..... yes
  Tracing backend ........................ <none>
  Logging backends:
    journald ............................. no
    syslog ............................... no
    slog2 ................................ no
  PCRE2 .................................. yes
    Using system PCRE2 ................... no
Qt Network:
  getifaddrs() ........................... yes
  IPv6 ifname ............................ yes
  libproxy ............................... no
  Linux AF_NETLINK ....................... yes
  OpenSSL ................................ yes
    Qt directly linked to OpenSSL ........ no
  OpenSSL 1.1 ............................ yes
  DTLS ................................... yes
  OCSP-stapling .......................... yes
  SCTP ................................... no
  Use system proxies ..................... yes
  GSSAPI ................................. no
Qt Gui:
  Accessibility .......................... yes
  FreeType ............................... yes
    Using system FreeType ................ yes
  HarfBuzz ............................... yes
    Using system HarfBuzz ................ no
  Fontconfig ............................. yes
  Image formats:
    GIF .................................. yes
    ICO .................................. yes
    JPEG ................................. yes
      Using system libjpeg ............... no
    PNG .................................. yes
      Using system libpng ................ yes
  Text formats:
    HtmlParser ........................... yes
    CssParser ............................ yes
    OdfWriter ............................ yes
    MarkdownReader ....................... yes
      Using system libmd4c ............... no
    MarkdownWriter ....................... yes
  EGL .................................... yes
  OpenVG ................................. no
  OpenGL:
    Desktop OpenGL ....................... yes
    OpenGL ES 2.0 ........................ no
    OpenGL ES 3.0 ........................ no
    OpenGL ES 3.1 ........................ no
    OpenGL ES 3.2 ........................ no
  Vulkan ................................. yes
  Session Management ..................... yes
Features used by QPA backends:
  evdev .................................. yes
  libinput ............................... no
  INTEGRITY HID .......................... no
  mtdev .................................. no
  tslib .................................. no
  xkbcommon .............................. yes
  X11 specific:
    XLib ................................. yes
    XCB Xlib ............................. no
    EGL on X11 ........................... no
    xkbcommon-x11 ........................ yes
QPA backends:
  DirectFB ............................... no
  EGLFS .................................. yes
  EGLFS details:
    EGLFS OpenWFD ........................ no
    EGLFS i.Mx6 .......................... no
    EGLFS i.Mx6 Wayland .................. no
    EGLFS RCAR ........................... no
    EGLFS EGLDevice ...................... no
    EGLFS GBM ............................ no
    EGLFS VSP2 ........................... no
    EGLFS Mali ........................... no
    EGLFS Raspberry Pi ................... no
    EGLFS X11 ............................ no
  LinuxFB ................................ yes
  VNC .................................... yes
Qt Sql:
  SQL item models ........................ yes
Qt Widgets:
  GTK+ ................................... no
  Styles ................................. Fusion Windows
Qt PrintSupport:
  CUPS ................................... no
Qt Sql Drivers:
  DB2 (IBM) .............................. no
  InterBase .............................. no
  MySql .................................. no
  OCI (Oracle) ........................... no
  ODBC ................................... no
  PostgreSQL ............................. no
  SQLite2 ................................ no
  SQLite ................................. yes
    Using system provided SQLite ......... no
  TDS (Sybase) ........................... no
Qt Testlib:
  Tester for item models ................. yes
Serial Port:
  ntddmodm ............................... no
Qt SerialBus:
  Socket CAN ............................. yes
  Socket CAN FD .......................... yes
  SerialPort Support ..................... yes
Further Image Formats:
  JasPer ................................. no
  MNG .................................... no
  TIFF ................................... yes
    Using system libtiff ................. no
  WEBP ................................... yes
    Using system libwebp ................. no
Qt QML:
  QML network support .................... yes
  QML debugging and profiling support .... yes
  QML just-in-time compiler .............. yes
  QML sequence object .................... yes
  QML XML http request ................... yes
  QML Locale ............................. yes
Qt QML Models:
  QML list model ......................... yes
  QML delegate model ..................... yes
Qt Quick:
  Direct3D 12 ............................ no
  AnimatedImage item ..................... yes
  Canvas item ............................ yes
  Support for Qt Quick Designer .......... yes
  Flipable item .......................... yes
  GridView item .......................... yes
  ListView item .......................... yes
  TableView item ......................... yes
  Path support ........................... yes
  PathView item .......................... yes
  Positioner items ....................... yes
  Repeater item .......................... yes
  ShaderEffect item ...................... yes
  Sprite item ............................ yes
QtQuick3D:
  Assimp ................................. yes
  System Assimp .......................... no
Qt Scxml:
  ECMAScript data model for QtScxml ...... yes
Qt Gamepad:
  SDL2 ................................... no
Qt 3D:
  Assimp ................................. yes
  System Assimp .......................... no
  Output Qt3D GL traces .................. no
  Use SSE2 instructions .................. yes
  Use AVX2 instructions .................. no
  Aspects:
    Render aspect ........................ yes
    Input aspect ......................... yes
    Logic aspect ......................... yes
    Animation aspect ..................... yes
    Extras aspect ........................ yes
Qt 3D Renderers:
  OpenGL Renderer ........................ yes
  RHI Renderer ........................... no
Qt 3D GeometryLoaders:
  Autodesk FBX ........................... no
Qt Wayland Drivers:
  EGL .................................... yes
  Raspberry Pi ........................... no
  XComposite EGL ......................... no
  XComposite GLX ......................... no
  DRM EGL ................................ yes
  libhybris EGL .......................... no
  Linux dma-buf server buffer integration . no
  Vulkan-based server buffer integration . yes
  Shm emulation server buffer integration . yes
Qt Wayland Client Shell Integrations:
  xdg-shell .............................. yes
  xdg-shell unstable v5 (deprecated) ..... yes
  xdg-shell unstable v6 .................. yes
  ivi-shell .............................. yes
  wl-shell (deprecated) .................. yes
Qt Wayland Client ........................ yes
Qt Wayland Compositor .................... yes
Qt Wayland Compositor Layer Plugins:
  VSP2 hardware layer integration ........ no
Qt Bluetooth:
  BlueZ .................................. no
  BlueZ Low Energy ....................... no
  Linux Crypto API ....................... no
  Native Win32 Bluetooth ................. no
  WinRT Bluetooth API (desktop & UWP) .... no
  WinRT advanced bluetooth low energy API (desktop & UWP) . no
Qt Sensors:
  sensorfw ............................... no
Qt Quick Controls 2:
  Styles ................................. Default Fusion Imagine Material Universal
Qt Quick Templates 2:
  Hover support .......................... yes
  Multi-touch support .................... yes
Qt Positioning:
  Gypsy GPS Daemon ....................... no
  WinRT Geolocation API .................. no
Qt Location:
  Qt.labs.location experimental QML plugin . no
  Geoservice plugins:
    OpenStreetMap ........................ yes
    HERE ................................. yes
    Esri ................................. yes
    Mapbox ............................... yes
    MapboxGL ............................. yes
    Itemsoverlay ......................... yes
QtXmlPatterns:
  XML schema support ..................... yes
Qt Multimedia:
  ALSA ................................... yes
  GStreamer 1.0 .......................... no
  GStreamer 0.10 ......................... no
  Video for Linux ........................ yes
  OpenAL ................................. no
  PulseAudio ............................. no
  Resource Policy (libresourceqt5) ....... no
  Windows Audio Services ................. no
  DirectShow ............................. no
  Windows Media Foundation ............... no
Qt TextToSpeech:
  Flite .................................. no
  Flite with ALSA ........................ no
  Speech Dispatcher ...................... no
Qt Tools:
  Qt Assistant ........................... yes
  Qt Designer ............................ yes
  Qt Distance Field Generator ............ yes
  kmap2qmap .............................. yes
  Qt Linguist ............................ yes
  Mac Deployment Tool .................... no
  makeqpf ................................ yes
  pixeltool .............................. yes
  qdbus .................................. yes
  qev .................................... yes
  Qt Attributions Scanner ................ yes
  qtdiag ................................. yes
  qtpaths ................................ yes
  qtplugininfo ........................... yes
  Windows deployment tool ................ no
  WinRT Runner Tool ...................... no
Qt Tools:
  QDoc ................................... yes
Qt WebEngine Build Tools:
  Use System Ninja ....................... no
  Jumbo Build Merge Limit ................ 8
  Developer build ........................ no
  Sanitizer .............................. no
  QtWebEngine required system libraries:
    fontconfig ........................... yes
    dbus ................................. no
    nss .................................. no
    khr .................................. yes
    glibc ................................ yes
  Optional system libraries used:
    re2 .................................. no
    icu .................................. no
    libwebp, libwebpmux and libwebpdemux . no
    opus ................................. no
    ffmpeg ............................... no
    libvpx ............................... no
    snappy ............................... no
    glib ................................. no
    zlib ................................. yes
    minizip .............................. no
    libevent ............................. no
    libxml2 and libxslt .................. no
    lcms2 ................................ no
    png .................................. yes
    JPEG ................................. no
    harfbuzz ............................. no
    freetype ............................. yes

Note: Also available for Linux: linux-clang linux-icc

Note: The following modules are not being compiled in this configuration:
    webenginecore
    webengine
    webenginewidgets
    pdf
    pdfwidgets

WARNING: Tool gperf is required to build QtWebEngine.

WARNING: Tool gperf is required to build QtPdf.

WARNING: QtWebEngine will not be built.

WARNING: QtPdf will not be built.

Qt is now configured for building. Just run 'gmake'.
Once everything is built, you must run 'gmake install'.
Qt will be installed into '/usr/local/Qt-5.15.12'.

Prior to reconfiguration, make sure you remove any leftovers from
the previous build.

```
<hr>

# qtdoc

```
To Generate Qt Documentation:

qtdoc contains the main Qt Reference Documentation, which includes
overviews, Qt topics, and examples not specific to any Qt module.The
configuration files are located in qtdoc/doc/config and the articles in
qtdoc/doc/src. Note that QDoc is located in qttools/src/qdoc.

The instructions in this file assumes that the prerequisite binaries are
compiled and in the $PATH variable.

Prerequisites:
    * qtbase exists
    * "qmake" and "qdoc" compiled and installed
    * other Qt repositories exist as needed

The Qt Reference Documentation were written with links to the released modules
for Qt 5. If the modules and repositories do not exist, then the resulting HTML
files will contain broken links.

There are two ways to build the documentation. One way is to compile the
documentation using QDoc and the configuration file (qdocconf) file. The other
way is to use qmake by running "make docs".

Section 1 Building the qdocconf File

    This method is useful for building separate projects without any
    dependencies to other projects.

    To build using the qdocconf file, run the "qdoc" binary and pass the
    qdocconf file as a parameter. "qdoc" is found in qttools repository. A
    mandatory "outputdir" must be specified.

        $> qdoc doc/config/qtdoc.qdocconf -outputdir html

    Note that QDoc will delete the contents of the "html" output directory.

Section 2 Building using make

    QMake uses the QT_INSTALL_DOCS variable to determine where the documentation
    for Qt 5 are installed. This method is useful for linking to other Qt
    modules.

    To see where the documentation will be installed, run:
    $> qmake -query

    To generate the documentation, run:

    $> cd qtdoc  #or whichever repository's root
    $> qmake
    $> make docs

    "make docs" also works in the main Qt 5 repository. Running the command
    there will generate the documentation for Qt 5 and install them to the
    path set to the QT_INSTALL_DOCS variable.

Section 3 Building Qt Documentation

    To build the Qt module documentation along with the Qt Reference Documentation,
    run the following:

    $> cd qt5  #the main qt5.git directory
    $> make qmake_all  #creates Makefiles for each repository
    $> make docs

    To generate only HTML files, run "make html_docs" instead of "make docs"

Section 4 Packaging the Documentation

    To package the documentation for Qt Assistant, the HTML files must be
    compiled into a QCH file (.qch).

    Required binaries:
    * assistant - found in qttools
    * qhelpgenerator - found in qttools

    To compile thq qch file for each module, first enter the output directory
    which contains the .qhp file and generate the QCH file.

    $> cd qtbase/doc/qtdoc              #the default path for QT_INSTALL_DOCS for qtdoc
    $> qhelpgenerator qtdoc.qhp         #creates the QCH file called qtdoc.qch

    Alternatively, modules have a "qch_docs" target:
    $> cd qtbase
    $> make qch_docs #builds the QCH files for modules in qtbase

    The QCH file can then be loaded in Qt Assistant or Qt Creator. For Qt
    Assistant, the QCH file may be registered to automatically load it.

    $> assistant -register qtdoc.qch    #to automatically load the documentation

Section 5 More Information

For more information about Qt 5's documentation, refer to the Qt Project wiki:
http://wiki.qt.io/Qt5DocumentationProject

```

- 한글 번역
```
Qt 문서를 생성하려면:

qtdoc에는 다음과 같은 주요 Qt Reference Documentation이 포함되어 있습니다
개요, Qt 토픽 및 Qt 모듈에 한정되지 않은 예제
구성 파일은 qtdoc/doc/config에 있으며 문서는
qtdoc/doc/src. QDoc은 qtools/src/qdoc에 있습니다.

이 파일의 지침은 필수 바이너리가 다음과 같다고 가정합니다
$PATH 변수에 컴파일되어 있습니다.

전제 조건:
* qtbase가 존재합니다
* "qmake" 및 "qdoc"을 컴파일하여 설치합니다
* 필요에 따라 다른 Qt 리포지토리가 존재합니다

Qt Reference Documentation은 출시된 모듈에 대한 링크와 함께 작성되었습니다
Qt 5의 경우 모듈과 리포지토리가 존재하지 않으면 결과 HTML
파일에는 끊어진 링크가 포함됩니다.

문서를 작성하는 데는 두 가지 방법이 있습니다. 한 가지 방법은 문서를 컴파일하는 것입니다
QDoc 및 구성 파일(qdocconf) 파일을 사용한 문서화, 기타
방법은 "make docs"를 실행하여 qmake를 사용하는 것입니다.

섹션1 qdocconf 파일 구축

이 방법은 별도의 프로젝트를 구축하는 데 유용합니다
다른 프로젝트에 대한 종속성.

qdocconf 파일을 사용하여 빌드하려면 "qdoc" 바이너리를 실행한 후
qdocconf 파일을 매개변수로 "qdoc"이 qtools repository에 있습니다. A
필수 "output dir"를 지정해야 합니다.

$> qdoc doc/config/qtdoc.qdocconf - 출력 dir html

QDoc은 "html" 출력 디렉토리의 내용을 삭제합니다.

제2절 메이크를 이용한 건축물

QMake는 QT_INSTALL_DOCs 변수를 사용하여 문서 위치를 확인합니다
Qt 5의 경우 설치되어 있습니다. 이 방법은 다른 Qt와의 연결에 유용합니다
모듈.

설명서가 설치될 위치를 확인하려면 다음을 실행합니다:
$> qmake - query

설명서를 생성하려면 다음을 실행합니다:

$> cd qtdoc # 또는 모든 저장소의 루트
$> qmake
$> 문서 만들기

"make docs"는 메인 Qt 5 저장소에서도 작동합니다. 명령어 실행
Qt 5에 대한 문서를 생성하고 이 문서를 다음과 같이 설치합니다
QT_INSTALL_DOCs 변수로 설정된 경로입니다.

섹션 3 빌딩 Qt 문서

Qt 참조 설명서와 함께 Qt 모듈 설명서를 구축하기 위해,
다음을 실행합니다:

$> cd qt5 #메인 qt5.git 디렉토리
$> make_all #create 각 저장소에 대한 파일 만들기
$> 문서 만들기

HTML 파일만 생성하려면 "make docs" 대신 "make html_docs"를 실행합니다

섹션 4 설명서 포장

Qt Assistant용 설명서를 패키지화하려면 HTML 파일이
QCH 파일(.qch)로 컴파일됩니다.

필수 바이너리:
* 보조 - qtool에서 발견됨
* qhelpgenerator - qtool에서 발견됨

각 모듈에 대해 thqqqch 파일을 컴파일하려면 먼저 출력 디렉토리를 입력합니다
.qhp 파일을 포함하고 QCH 파일을 생성합니다.


```

