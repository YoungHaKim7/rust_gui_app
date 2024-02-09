# QT5 Install

- https://www.linuxfromscratch.org/blfs/view/svn/x/qt5.html


```
./Configure

Qt is now configured for building. Just run 'gmake'.
Once everything is built, you must run 'gmake install'.
Qt will be installed into '/usr/local/Qt-5.15.12'.

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

<hr>

# cxx-qt
- Safe interop between Rust and Qt

  - https://github.com/KDAB/cxx-qt

# The CXX-Qt Book walks through a minimal example step-by-step and documents CXX-Qt's features for the latest release.

- https://kdab.github.io/cxx-qt/book/getting-started/index.html

  - Getting Started - CXX-Qt Documentation
    - 이번에 투자 받았다고 하는곳 c++ 코드 => 러스트 코드로 변환 작업 중 :rust~1:

# macos QT

https://doc.qt.io/qt-6/macos.html
