# Xilem
- An experimental Rust native UI framework
- https://github.com/linebender/xilem

  - Rust 용 Data-first UI 디자인 툴킷인 Druid 를 만들었던 사람의 제안 글이네요. 꽤 긴글이라 도입부만 옮겨봅니다. 결과물이 나와봐야 알 것 같아요. Raph Levien은 사실 Druid만으로 말하기에는 너무 부족한 사람인데... (GPL) Ghostscript 메인테이너, Advogato 블로그 커뮤니티, Inconsolata 글꼴, Xi 편집기 등으로 예전부터 유명합니다.
    - https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html
    - https://news.hada.io/topic?id=6519

![1](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/e550734d-059b-4953-a011-9b315c9c8fa2)
- https://youtu.be/OvfNipIcRiQ?si=vn6pX8pVNB1dqSYq
  
# 최신글 
- 240119 https://linebender.org/blog/xilem-backend-roadmap/

<hr>

- (240605)Xilem: Let's Build High Performance Rust UI - Raph Levien | Rust Nederland (RustNL)
  - https://youtu.be/OvfNipIcRiQ?si=vn6pX8pVNB1dqSYq
    - 강의내용 github https://github.com/rustnl/meetups/tree/main/20240507-delft/slides


Ralph의 발표에 큰 박수를 보내 주세요. 감사합니다.

- We're working on this project of building high-performance UI. It's under the Xilin kind of umbrella for the project, but I'm going to talk about, I'm going to give an overview of the pieces of it, but the call to action here is come and get involved in the project.
ChatGPT
  - 우리는 고성능 UI를 구축하는 프로젝트를 진행 중입니다. 이 프로젝트는 Xilin의 일종의 프로젝트 안에 있지만, 저는 여기서 그 구성 요소에 대한 개요를 제공할 거에요. 그러나 여기서의 콜 투 액션은 이 프로젝트에 참여해 주세요.

- So first, I want to talk a little bit about the goals of the project. So the real focus of this entire thing is high performance. And that's actually a little bit of an unusual goal. Most of the time, you're more focused on developer experience, and so on and so forth. But we really feel like high performance is the niche. This is why you would use Rust to build UI.
ChatGPT
  - 먼저, 이 프로젝트의 목표에 대해 조금 이야기하고 싶습니다. 이 전체 프로젝트의 진정한 초점은 고성능입니다. 실제로 이것은 조금 이상한 목표입니다. 대부분의 경우에는 개발자 경험에 더 집중하는 편이지만, 우리는 고성능이 중요하다고 생각합니다. 이것이 Rust로 UI를 구축하는 이유입니다.
    
- That's driving a lot of the decisions that we're making. We're also focused on a very rich 2D graphics model. We want to have full access to vector graphics with all of the goodies like gradients and also really good font support. I'll talk about that a little bit more. Of course, that's what I do. But we don't want to just have something that's high-performance and rich but very hard to program. We also want to make a really ergonomic, easy-to-use API on top of that. And we want this to work on desktop and mobile. The project is being funded by Google. It's a research project that's being funded by Google Fonts. And the focus of that is Android apps. So we want to do both. We want to demonstrate a really good experience on Android, but I think the community is interested in desktop and other targets as well.
ChatGPT
  - 이러한 결정에 많은 영향을 미치고 있습니다. 또한 매우 풍부한 2D 그래픽 모델에도 집중하고 있습니다. 우리는 그래디언트와 같은 모든 기능을 갖춘 벡터 그래픽에 완전한 액세스를 원합니다. 또한 정말 좋은 글꼴 지원도 필요합니다. 좀 더 자세히 이야기하겠습니다. 물론, 그게 제가 하는 일이죠. 하지만 우리는 고성능과 풍부함만 가지고 있어서 프로그래밍이 매우 어렵다면 안 되겠습니다. 또한, 그 위에 정말 편안하고 사용하기 쉬운 API를 만들고 싶습니다. 그리고 이것을 데스크톱과 모바일에서 작동하도록 하고 싶습니다. 이 프로젝트는 Google에서 자금을 지원하고 있습니다. 이는 Google Fonts에서 자금을 지원하는 연구 프로젝트입니다. 이 프로젝트의 초점은 Android 앱입니다. 그래서 우리는 둘 다 하려고 합니다. Android에서 좋은 경험을 보여주고 싶지만, 커뮤니티는 데스크톱과 다른 타겟에도 관심이 있습니다.


- This is a research project. The goals of this, we're not offering this as a product, you know, something that's fully ready to use out of the box. Maybe it will evolve into that over time, but the goals for right now are to discover, like, what is the best way to build UI in Rust and, you know, to what extent are we able to realize these goals of being extremely high performance and fully featured as a user interface.
ChatGPT

  - 이것은 연구 프로젝트입니다. 이 프로젝트의 목표는 제품으로 제공되는 것이 아니라, 상자에서 꺼내서 즉시 사용할 수 있는 것이 아닙니다. 시간이 지나면 이것이 그렇게 진화될 수도 있지만, 현재의 목표는 Rust로 UI를 구축하는 가장 좋은 방법을 발견하는 것입니다. 그리고 우리가 얼마나 고성능이고 기능이 풍부한 사용자 인터페이스를 실현할 수 있는지에 대한 범위입니다.


- We want to work with the community. This is maybe a little bit of a newer focus of the project, that we've been maybe a little bit siloed, maybe doing some of our own things. But as I go through the slides, you'll see that there are a number of decisions that we've made about how do we cooperate with the Rust ecosystem instead of just doing our own thing.
ChatGPT
  - 우리는 커뮤니티와 함께 일하고 싶습니다. 이것은 프로젝트의 조금 새로운 초점일 수 있습니다. 우리는 조금 격리되어 있었거나 자체적인 일을 하고 있을 수도 있습니다. 그러나 저는 슬라이드를 통해 진행하면서 우리가 어떤 결정을 내렸는지 볼 수 있을 것입니다. Rust 생태계와의 협력에 대해 어떻게 할 것인지에 대한 것이죠.


- This is kind of a picture of what the problems that you need to solve in UI, and I want to point people to Nico. Nico Burns did a talk on Monday, and there's also a white paper that's coming out soon, and that work is giving a much more complete picture of what you're seeing on the screen here. But we are seeing the problem in layers. So there's a top layer, which is reactivity, and this could be provided, like each of these layers, each of these boxes could be provided by a number of different crates. So this reactivity could be provided by Dioxys or Leptos, and then I'll be talking a lot more about what we're doing with this. And then there's a middle layer, which is solving a lot of the guts of what's involved in UI, of having a full collection of widgets and having gestures and layout and your text editing. There's a lot of problems that need to be solved at that layer. And then there's also a really important layer at the bottom of infrastructure. How do you connect to the platform, and how do you do the basic functions of drawing, text rendering, getting to your GPU, and accessibility? I'll be talking a lot more about that.
ChatGPT
  - 이것은 UI에서 해결해야 할 문제들의 그림이며, Nico에게 주목하고 싶습니다. Nico Burns가 월요일에 발표를 했고, 곧 나올 화이트 페이퍼도 있습니다. 그 작업은 여기 화면에서 보이는 것의 훨씬 완전한 그림을 제공합니다. 하지만 우리는 문제를 층별로 보고 있습니다. 맨 위층은 반응성입니다. 이것은 각각의 층, 각각의 상자들이 여러 다른 크레이트로 제공될 수 있습니다. 이 반응성은 Dioxys나 Leptos에 의해 제공될 수 있으며, 저는 이에 대해 더 자세히 이야기하겠습니다. 그리고 중간층에는 UI의 내용을 해결하는 많은 내부 구조가 있습니다. 위젯의 전체 컬렉션과 제스처 및 레이아웃 및 텍스트 편집을 가지고 있는 것과 관련된 많은 문제들이 있습니다. 그리고 매우 중요한 하단의 인프라 레이어도 있습니다. 플랫폼에 연결하는 방법과 그래픽 처리 장치에 도달하는 방법, 그리고 접근성은 어떻게 하는지에 대해 더 많은 이야기를 나눌 것입니다.


- So this is kind of our instantiation of this layered architecture. And so at the top level, we have the Xilin Reactive Architecture. In the middle layer, we have Masonry, which is our widget set, which does not depend on Xilin. So you could imagine using Masonry with Deoxys or Leptos. And then we're using Taffy for layout. And at the bottom, we have a mixture of things that are Rust ecosystem, like WinIT. We made a decision. We switched over in January from our own platform integration to using WinIT. And then a lot of the other things are things that we're building, that obviously we're building for use in Xilin, but we're also hoping can be used by other projects in the Rust ecosystem.
ChatGPT

- 이것은 저희가 층별 아키텍처를 구현한 것입니다. 맨 위층에는 Xilin 반응형 아키텍처가 있습니다. 중간층에는 Xilin에 의존하지 않는 위젯 세트인 Masonry가 있습니다. 그래서 Masonry를 Deoxys나 Leptos와 함께 사용할 수 있다고 상상할 수 있습니다. 그리고 레이아웃에는 Taffy를 사용하고 있습니다. 맨 아래층에는 WinIT과 같은 Rust 생태계의 혼합물이 있습니다. 우리는 1월에 우리 자체의 플랫폼 통합에서 WinIT을 사용하도록 전환하는 결정을 내렸습니다. 그리고 다른 많은 것들은 우리가 구축 중인 것들입니다. 분명히 Xilin에서 사용하기 위해 구축 중이지만, Rust 생태계의 다른 프로젝트에서도 사용될 수 있기를 희망합니다.


- Again, Xylem is generic. There's a Xylem core, which is generic over the element tree that it's driving, so we can drive masonry widget set and also give a demo of using the Xylem core to drive DOM and create experience that can run in the browser.
ChatGPT

  - 다시 말해, Xilem은 범용적입니다. Xilem 코어는 그것이 구동하는 요소 트리에 대해 범용적입니다. 그래서 우리는 Masonry 위젯 세트를 구동할 수 있고, 또한 브라우저에서 실행할 수 있는 경험을 만들기 위해 Xilem 코어를 사용하여 DOM을 구동하는 데모를 제공할 수 있습니다.

- So the Xilin architecture, the Xilin Reactive architecture, gives you very simple code. Like this is your classic counter example, and this is really all of the code that there is. This is the actual code that I'm going to run. So it's a pretty simple code to get this functionality. And if I run this,
ChatGPT
  - Xilin 아키텍처, Xilin 반응형 아키텍처는 매우 간단한 코드를 제공합니다. 이것은 전형적인 카운터 예제이며, 실제로 모든 코드가 여기에 있습니다. 이것이 실행할 실제 코드입니다. 이 기능을 얻으려면 매우 간단한 코드입니다. 실행하면,


- Pray to the demo gods, we have a usable example. Very simple. And I'll tempt the demo gods a little bit more and do a little bit of live coding. Let's add a checkbox to this. Also add a checkbox and test my live typing skills. And we'll call it is zero. And we'll be checked if count is actually equal to zero. And then on the action, we've got the app state, which in this case is just a count. And we've got a Boolean for whether it's checked. And we'll just set the count to zero or one based on sort of the opposite of the checked state. And we'll coerce that to 32. And this should give an error because checkbox, I'll just add it.
ChatGPT
  - 신의 이름으로 데모를 기도해봅니다. 우리에게 사용 가능한 예제가 있습니다. 매우 간단합니다. 그리고 조금 더 데모를 유혹하고 실시간 코딩도 해보겠습니다. 이것에 체크박스를 추가해 봅시다. 타이핑 실력을 테스트하고 체크박스를 추가해 보겠습니다. 이를 'is zero'라고 부르겠습니다. 그리고 count가 실제로 0과 같은 경우에만 체크되도록 하겠습니다. 그리고 액션에는 이 경우 앱 상태가 있으며, 이는 단순히 카운트입니다. 그리고 체크 여부에 대한 불리언이 있습니다. 그리고 체크된 상태의 반대에 따라 카운트를 0 또는 1로 설정하겠습니다. 그리고 이를 32로 강제 변환할 것입니다. 이것은 checkbox가 없기 때문에 에러가 발생해야 합니다. 간단히 추가하겠습니다.


- And this is a Mac M1, so this is going to be a lot slower than what you saw yesterday morning, but it's pretty fast still. And now we've got a checkbox there, and we can use that. You know, it's reactive, so if you change the count here, reset it, it's changing the checkbox state because that's all kind of computed in the app logic. It's here.
ChatGPT
  - 이건 맥 M1이라서 어제 아침에 보았던 것보다 훨씬 느릴 거예요. 그러나 여전히 꽤 빠릅니다. 이제 체크박스가 있고, 이를 사용할 수 있습니다. 반응형이기 때문에 여기서 카운트를 변경하면 체크박스 상태도 변경되며, 이는 모두 앱 로직에서 계산됩니다. 여기 있습니다.

- All right, back to the slides. So yeah, there's a few things that are a little bit different than a lot of the reactive layers that you see are derived from the JavaScript world, which is typically very dynamically type. And I think Xylem is a lot more inspired by Swift UI than it is by JavaScript frameworks. So everything that you're seeing, all of these views that are created are statically type. And that also means that the process of reconciliation, of dissing, this is all working statically typed data. So it's going to be a lot more efficient than doing something that's dynamically type. You'll also notice looking at that code that there are no macros in there, that these are actually fairly simple Rust value data types. Like, you know, when there's a string, well, there's no label in here. I'm not going to add a label, life-coded, but it's really just a string. And then that string is a view that creates the underlying label widget, but the programmer doesn't really have to worry about that.
ChatGPT
  - 좋아요, 다시 슬라이드로 돌아가 보겠습니다. 그래서 많은 반응형 레이어들이 일반적으로 JavaScript 세계에서 유래되었는데, 이는 보통 매우 동적으로 타입됩니다. 그리고 Xilem은 JavaScript 프레임워크보다는 Swift UI에서 많은 영감을 받았다고 생각합니다. 그래서 보고 계신 모든 뷰들은 정적으로 타입됩니다. 그리고 이는 모든 재조정 및 해결 과정이 정적으로 타입된 데이터에서 작동하기 때문에, 동적으로 타입된 것을 수행하는 것보다 훨씬 효율적일 것입니다. 그 코드를 보면 거기에 매크로가 없다는 것을 알 수 있습니다. 이것들은 실제로 상당히 간단한 Rust 값 데이터 유형입니다. 예를 들어, 문자열이 있는 경우, 여기에는 레이블이 없습니다. 레이블을 추가하지 않겠습니다. 라이브 코딩하는 것은 아니지만, 이것은 정말로 그냥 문자열입니다. 그리고 그 문자열은 기본 레이블 위젯을 생성하는 뷰이지만, 프로그래머는 실제로 그것에 대해 걱정할 필요가 없습니다.


- So as I mentioned, there's a generic Xilin core. In this case, we're generating a masonry view, which is our native widget type. But you could also generate other things like web, and I'll show an example of that.
ChatGPT

  - 제가 언급했듯이, 범용 Xilem 코어가 있습니다. 이 경우에는 Masonry 뷰를 생성하고 있습니다. 이는 우리의 네이티브 위젯 유형입니다. 그러나 웹과 같은 다른 것들도 생성할 수 있습니다. 그것의 예를 보여드리겠습니다.


- XIALM is based at the core of XIALM. Drawing is really essential to any GUI toolkit, and that's also where you tend to spend most of the CPU time, most of the performance. It's just how fast can you get this thing drawn onto the screen. And so that's a lot of what I've been doing over the past few years, really, is building a fast 2D rendering engine that does as much of the work as possible on the GPU. You want to do as little as possible on the CPU. You want to offload all of that work to the GPU. So it is implementing the full standard 2D imaging model. So we have vector paths with fills and strokes, gradients, images, clips, blends, and text. Let's go to the demo.
ChatGPT
  - XIALM은 XIALM의 핵심에 기반합니다. 그리기는 모든 GUI 툴킷에 매우 중요하며, 대부분의 CPU 시간, 대부분의 성능을 사용하는 곳입니다. 화면에 이것을 얼마나 빨리 그릴 수 있는가가 중요합니다. 그래서 지난 몇 년 동안 저는 가능한 한 많은 작업을 GPU에서 수행하는 빠른 2D 렌더링 엔진을 구축하는 데 주력해 왔습니다. CPU에서 최소한의 작업을 수행하고 가능한 많은 작업을 GPU로 오프로드하려고 합니다. 그래서 표준 2D 이미징 모델을 완전히 구현하고 있습니다. 따라서 우리는 벡터 경로와 채우기 및 스트로크, 그래디언트, 이미지, 클립, 블렌드 및 텍스트를 사용할 수 있습니다. 데모로 이동해 보죠.

- Um, let's see if we can...
ChatGPT
  - 제가 도와드릴 수 있는 게 있나요? 무엇을 하고 싶으세요?


- So, I'm just going to run through kind of our standard test scene so you always have the...
ChatGPT
  - 그러니까, 제가 표준 테스트 장면을 실행해볼게요. 그래서 항상...


- You need to have that. Some of these things, like we have a lot of things in there for stroking, because just in the past few months we've moved from the stroke expansion being computed on the CPU to entirely being computed on the GPU. And actually there's a paper that I just submitted to a conference, so you can see the preprint. There's a lot of math that goes into computing the stuff on the GPU.
ChatGPT
  - 그걸 갖고 있어야 해요. 최근 몇 달 동안 스트로크 확장이 CPU에서 계산되는 것에서 완전히 GPU에서 계산되도록 이동했기 때문에 그 중 일부는 CPU에서 계산되는 것이 있습니다. 사실, 제가 방금 학회에 제출한 논문이 있으니까 프리프린트를 볼 수 있어요. GPU에서 이런 것들을 계산하는 데는 많은 수학이 필요합니다.

- Mention the turn v-sync off. Things go a lot faster. Here's a fun test scene that has, you know, a little bit, this is actually not a lot of strokes. We have test scenes in here as, you know, tens of thousands of strokes. And so this one is showing a bunch of things. It's showing the animation and an image in there. And one of the things that we're also showing here is that our text and font handling is able to do variable fonts. And one of the goals, one of the things that's kind of driving this is to be able to do richer things like animate those variable font parameters. So you can go from light to bold, and you can animate the widths completely dynamically. And that's a, to get really high performance, this would be a major challenge with existing TD graphics libraries. So this is just kind of going through, this is our engineering test scene. So here you can see all the different blend modes. There's more coming up that are sort of stress tests for how many line segments that you can have and still get decent performance. Yeah, this is another one. This is the motion mark example. And let's see with the stats that you can ramp up to, I think it goes up to like 70,000 and still running it over 120 frames a second. So the performance, moving all of this from the CPU to the GPU just unlocks hugely increased performance. I'll show you another example that really shows off their performance. This is one of our standards test scenes. It's a map of Paris. It has
ChatGPT
  - Async를 끄면 성능이 훨씬 빨라집니다. 여기 재미있는 테스트 장면이 있어요. 이것은 실제로 많은 수의 스트로크가 아니에요. 여기에는 수천 개의 스트로크가 있는 테스트 장면도 있습니다. 그래서 이것은 여러 가지를 보여줍니다. 애니메이션과 이미지를 보여주고 있습니다. 여기서 우리가 보여주고 있는 다른 것 중 하나는 텍스트와 글꼴 처리가 변수 글꼴을 다룰 수 있다는 것입니다. 이것을 통해 변수 글꼴 매개변수를 애니메이션화할 수 있는 등 보다 풍부한 기능을 수행할 수 있는 것이 목표 중 하나입니다. 예를 들어 글꼴 두께를 가볍게 바꾸거나 진하게 바꾸고, 너비를 완전히 동적으로 애니메이션화할 수 있습니다. 기존의 2D 그래픽 라이브러리로는 이를 구현하는 것이 매우 어려운 과제일 것입니다. 여기 엔지니어링 테스트 장면이 있습니다. 다양한 블렌드 모드를 모두 볼 수 있습니다. 좀 더 나오는 것들은 성능을 얼마나 향상시킬 수 있는지에 대한 스트레스 테스트입니다. 이것은 또 다른 예제입니다. 이것은 모션 마크 예제입니다. 그리고 통계를 보면, 70,000까지 올라가고도 여전히 초당 120프레임으로 실행됩니다. 모든 것을 CPU에서 GPU로 옮기면 성능이 크게 향상됩니다. 다른 예를 보여드리겠습니다. 이것은 우리의 표준 테스트 장면 중 하나입니다. 파리 지도입니다.

- A million path segments in the image. You can zoom in, you can see there's a lot of detail there. And, you know, when we're V-sync off, you get hundreds of frames per second. And again, with standard rendering thing, it's challenging even to get 60 frames a second out of this particular test scene. So that's really a lot of what the Xylem project is about, is how do you exploit, if you have all that power, if you have all that graphical power, how do you exploit that from a REST UI? So there's a lot more to be said, including the multi-threaded encoding. Like, if you're rendering a scene of that richness and complexity, and you're only able to get that data to the GPU from a single thread, that can easily be the bottleneck. So one of the goals of this project is, have the different parts of the UI running in different threads, and have it assemble all these scene fragments together. So there's a lot of technical detail. I actually have a talk that I was gonna put a link, I forgot to do that. I did a talk at REST Lab last November, and I go into a lot more detail about the Velo engine. So if you're interested, go check out that talk.
ChatGPT
  - 이미지에는 백만 개의 경로 세그먼트가 있습니다. 확대해서 볼 수 있습니다. 많은 디테일이 있습니다. 그리고 V-sync를 끄면 초당 수백 프레임을 얻을 수 있습니다. 그리고 표준 렌더링에서는 이 특정 테스트 장면에서 60프레임을 얻는 것도 어려울 정도입니다. 그래서 Xilem 프로젝트의 주된 목적 중 하나는 만약 그만한 파워가 있다면, 그만한 그래픽 파워가 있다면, 이것을 어떻게 활용할 것인가에 대한 것입니다. 다양한 기술적 세부 사항이 있습니다. 실제로 저는 작년 11월에 REST Lab에서 발표한 것이 있는데, 그때 Velo 엔진에 대해 더 많은 세부 정보를 다루었습니다. 만약 관심이 있다면, 그 발표를 확인해 보세요.

- So the way that we use Velo in Xilin is also interesting because this is a different API. This is not having a Canvas context that has an in-mute reference to it, which would restrict you to a single thread. So we have a model in which the widget in Xilin retains a scene fragment. So the scene fragment is kind of, you can think of a scene fragment as like a snippet of binary SVG. So if you're painting, you don't have to repaint the whole widget every time. Then you can retain that scene fragment and only re-encode the subtree when it's changed. And so you can do things like change the transform. So if you're doing scrolling, you don't have to be re-encoding all the content of what's inside that clip window. You can just reuse that and then have the GPU do all the work of re-rendering it. So it's all about making the CPU do as little as possible and making the GPU do the work where it can be actually about 10 times as efficient in terms of how much you can get per second or how much throughput you can get per watch.
ChatGPT
  - Xilem에서 Velo를 사용하는 방식도 흥미롭습니다. 이것은 다른 API입니다. 이것은 Canvas 컨텍스트를 가지고 있지만 해당 컨텍스트에 대한 내부 참조가 있는 것이 아니기 때문에, 이것은 단일 스레드로 제한되지 않습니다. Xilem에서는 위젯이 장면 조각을 보유하도록 모델을 가지고 있습니다. 장면 조각을 이해하는 방법은 바이너리 SVG의 일부 조각으로 생각할 수 있습니다. 따라서 그림을 그릴 때마다 전체 위젯을 다시 그리지 않아도 됩니다. 그런 다음 그 장면 조각을 보존하고 하위 트리가 변경될 때만 재인코딩할 수 있습니다. 그래서 변형을 변경할 수 있습니다. 따라서 스크롤링을 할 때 스크롤 창 안에 있는 내용의 모든 내용을 다시 인코딩할 필요가 없습니다. 해당 내용을 재사용하고 GPU에게 모든 렌더링 작업을 수행하도록 할 수 있습니다. CPU가 최소한의 작업을 수행하고 GPU가 작업을 처리할 수 있는 곳에서 실제로는 초당 얼마나 많은 양을 얻을 수 있는지 또는 시간당 얼마나 많은 처리량을 얻을 수 있는지 약 10배 효율적입니다.

- Another one of the layers, or another one of the bricks in that image, text is obviously really central to any GUI. And so we have this new crate. And all these crates, by the way, that you saw, the pictures that you see here in bold, most of these just got released in the last two or three weeks. Some of them, I think the Xylem crate actually got released at midnight, so yesterday or today. Alon, in preparation for this talk, of course, we've been working on it for a very long time, but Parley is another one that got released, I think maybe two or three weeks ago. And it's an advanced text layout engine. And so it does the basics of text layout. So it does paragraph layout with line breaking. It has the ability to set attributes. You can do bold and italic and all that fun stuff. But it goes, and of course, it's designed for performance, really really metal performance. But we're also trying to do some richer things. So it supports variable fonts. And one of our goals, we don't have it, what you saw in the demo was not using Parley, I don't think. But we want to support variable fonts. We want to support the animation of variable font access and do richer interactions in the UI. Parley itself, of course, works well with Bellow, but it is not tied to a specific 2D rendering image. So this is one of these blocks that you can mix and match however you like. There's some details here. It's depending, there's two crates that are kind of below Parley. One is called Fontique, and Fontique accesses the system fonts and enumerates them and figures out which font is needed to render which script. And there's also a crate called Screefa, which does the low level font loading. So this is really the Rust alternative to FreeType. I think one of the most exciting things about Screefa is that there's work in progress to get this integrated into Chrome. So that's actually in a Chrome Canary build, I believe under a flag. But the goal is to make that really production quality to support all of the font loading that kind of low level font needs that Chrome has, which is a pretty big requirement.
ChatGPT
  - 텍스트는 GUI에서 아주 중요한 요소입니다. 그래서 우리는 이 새로운 크레이트를 가지고 있습니다. 여기서 보이는 그림들은 대부분이 지난 두 ~ 세 주 동안 출시된 것입니다. 일부는 실제로 지난 밤 자정에 출시된 것 같습니다. 이 토크를 준비하는 동안 아론은 물론 오랜 시간동안 작업해 왔지만, Parley도 이번에 출시된 것 중 하나입니다. 고급 텍스트 레이아웃 엔진입니다. 기본적인 텍스트 레이아웃을 수행합니다. 문단 레이아웃과 줄 바꿈을 수행합니다. 속성을 설정할 수 있습니다. 볼드체와 이탤릭체 등 다양한 속성을 사용할 수 있습니다. 물론 성능을 위해 설계되었습니다. 매우 빠른 성능입니다. 그러나 더 풍부한 기능을 구현하려고 노력하고 있습니다. 그래서 변수 글꼴을 지원합니다. 변수 글꼴 액세스의 애니메이션과 더 풍부한 UI 상호 작용을 지원하려고 합니다. Parley 자체는 Bellow와 잘 작동하지만, 특정 2D 렌더링 이미지에 결합되지 않았습니다. 따라서 원하는 대로 혼합 및 일치시킬 수 있는 이러한 블록 중 하나입니다. 여기에는 세부 정보가 있습니다. Parley 아래에 두 개의 크레이트가 있습니다. 하나는 Fontique이라고 하며, 시스템 글꼴에 액세스하고 필요한 글꼴을 렌더링하는 데 필요한 글꼴을 찾습니다. 그리고 Screefa라는 크레이트가 있으며, 이는 저수준 글꼴 로딩을 수행합니다. 이는 실제로 FreeType에 대한 Rust 대안입니다. Screefa에 대한 가장 흥미로운 점 중 하나는 Chrome에 통합되는 작업이 진행 중이라는 것입니다. 실제로 Chrome Canary 빌드에서 이를 확인할 수 있습니다. 하지만 그 목표는 실제로 프로덕션 품질을 갖추고, Chrome이 필요로 하는 모든 글꼴 로딩을 지원하는 것입니다.

- Another major goal of our project is to support accessibility well out of the box. And so we're funding work on AccessKit, which is, I think, the Rust ecosystem create for doing accessibility. And so it's a portable abstraction layer. Each platform has its own API for how it exposes accessibility. And so the Mac and Windows and Linux, more recently, backends are working pretty well. Android is in progress. That's one of our goals, to get that working. AccessKit is, of course, written in Rust, but there are bindings. We want to make that available for other languages. And there's some technical details I'll skip past and just go straight to the demo.
ChatGPT
  - 프로젝트의 또 다른 주요 목표 중 하나는 기본적으로 잘 알려진 접근성을 지원하는 것입니다. 그래서 우리는 AccessKit에 대한 작업을 후원하고 있습니다. 이것은 접근성을 처리하기 위한 Rust 생태계 크레이트입니다. 따라서 이는 휴대용 추상화 계층입니다. 각 플랫폼은 접근성을 노출하는 방식에 대한 고유한 API를 갖고 있습니다. 그래서 Mac, Windows, 그리고 리눅스 백엔드는 최근에 잘 작동하고 있습니다. 안드로이드는 진행 중입니다. 그것이 우리의 목표 중 하나입니다. 작동하도록 하는 것입니다. AccessKit은 물론 Rust로 작성되었지만, 바인딩이 있습니다. 이를 다른 언어로 이용할 수 있도록 하고 싶습니다. 몇 가지 기술적인 세부 사항은 건너뛰고 바로 데모로 이동하겠습니다.

- So we'll actually use that same simple example, and I'll turn it on. I'll go into the settings here, and I think I'll probably turn down the offer to update my software and go into accessibility and just turn on voiceover. Great. Fantastic.
ChatGPT
  - 그래서 실제로 같은 간단한 예제를 사용할 거에요. 그리고 접근성을 켜볼 거에요. 설정으로 들어가서 여기서 나의 소프트웨어를 업데이트할 것을 거절하고 접근성로 들어가서 VoiceOver를 켜볼 거에요. 좋아요. 훌륭해요.
16분 41초까지~~~






<hr>



39분 부터~~~
- We're very happy to be working with Matt Campbell, who is the real creator of AccessKit.
ChatGPT

  - 저희는 AccessKit의 실제 제작자인 Matt Campbell과 함께 일하게 되어 매우 기쁩니다.


- in combination with async, so how would you for example make an HTTP request?
ChatGPT
  - async와 결합해서, 예를 들어 HTTP 요청을 어떻게 만들까요?

- We just ripped the async out, which is fun. But we had a prototype where we had async integrated fairly deeply, so you have the ability to do waking, which is, of course, the primary point at which async integrates with a UI toolkit on a per-widget basis. So, again, this is a prototype that we had working, and then when we were making various different transitions to win it, that was part of the thing that we ended up taking that out. But we are planning on bringing that back, so you absolutely would be able to have a widget that says, you know,
ChatGPT
  - 저희는 방금 async를 제거했는데, 재미있었어요. 그러나 async가 꽤 깊이 통합된 프로토타입이 있었어요. 그래서 UI 툴킷과 개별 위젯 단위로 통합되는 주요 지점에서 깨우기 기능을 수행할 수 있었죠. 그래서, 다시 말하지만, 이건 저희가 작동하던 프로토타입이었고, 다양한 전환을 하면서 그것을 제거하게 됐어요. 하지만 저희는 그것을 다시 도입할 계획이 있어서, 위젯이 그런 기능을 가지게 될 수 있을 겁니다.

- DAFA network request, and then paint the widget based on the loading state while you're waiting for the request to come back. And so there's a prototype. I can point people to the code based on kind of an older version. This is why it's research. It's like we need to figure out the best way for all of these pieces to come together.
ChatGPT

  - DAFA 네트워크 요청을 하고, 요청이 돌아오는 동안 로딩 상태를 기반으로 위젯을 렌더링하는 거죠. 그래서 그런 프로토타입이 있습니다. 이전 버전을 기반으로 코드를 보여드릴 수 있습니다. 이런 이유로 연구가 필요한 겁니다. 모든 요소가 어떻게 가장 잘 결합될 수 있는지를 찾아내야 하니까요.

- but it's planned.
ChatGPT

  - 하지만 계획되어 있습니다.

- I think this is the final question.
ChatGPT
  - 이게 마지막 질문인 것 같네요.

- Hello, I've also been following your work for some time and I remember in Druid there was this a bit bizarre layer of integration with the system like file dialogues and application persistence with window closing. What did you do with that in the new Xilib architecture?
ChatGPT
  - 안녕하세요, 저는 당신의 작업을 오랫동안 지켜봐 왔고, Druid에서는 파일 대화 상자와 창 닫기 시 애플리케이션 지속성과 같은 시스템 통합 계층이 좀 이상하게 보였던 걸로 기억합니다. 새로운 Xilib 아키텍처에서는 그것을 어떻게 처리하셨나요?

- Well, I mean, the platform integration, we've just switched over to WinIT, and that does mean that we have a regress, so there is not a file open dialogue in Xilem at the moment. And so we do plan on porting that stuff on top of WinIT. I think there's some really good questions of, you know, do we do that in a way that's specific to Xilem, or do we try and make a more general crate that could be used by other, maybe other things in the Rust UI ecosystem? I think that's a little bit open. Like, the current state is, you know, we're trying to get the new infrastructure really functioning, and then pull in pieces from the kind of previous Druid. Yeah, one.
ChatGPT
  - 글쎄요, 플랫폼 통합에 대해서는, 저희가 WinIT로 전환했는데, 그로 인해 퇴보가 있었습니다. 현재 Xilem에는 파일 열기 대화 상자가 없어요. 그래서 그것을 WinIT 위에 이식할 계획입니다. Xilem에만 특화된 방식으로 할지, 아니면 Rust UI 생태계의 다른 요소들도 사용할 수 있는 더 일반적인 크레이트로 만들지에 대한 좋은 질문이 있다고 생각합니다. 이는 조금 열려 있는 부분인 것 같아요. 현재 상태는 새로운 인프라를 제대로 작동하게 만든 후, 이전 Druid에서 일부 요소를 가져오는 것입니다. 네, 그렇습니다.

- TouchGrade was mentioned earlier today. I don't remember the name, but yeah.
ChatGPT
  - 오늘 아침에 TouchGrade가 언급되었어요. 이름은 기억나지 않지만, 맞아요.
Okay, so give a huge hand to Ralph for his talk. Thank you.


<hr>
