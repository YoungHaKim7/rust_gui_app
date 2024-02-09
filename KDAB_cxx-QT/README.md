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

