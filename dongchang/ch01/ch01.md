# Before Start

간단하게 러스트를 배우게 된 이유

1. WebAssembly의 등장 (생각보단 오래되긴 했지만)
   추후 WebAssembly를 이용한 웹 프로젝트를 진행해보고 싶은 마음이 있다.
2. Solana나 Polkadot과 같이 블록체인 프로그램을 적어보고 싶어서
3. 아무래도 Javascript보다는 Low-Level이면서, 더 엄격하다보니
   새로운 프로그래밍 컨셉과 코딩 스타일을 배울 수 있을 것 같아서

# Hello World

```rust
fn main() {
    println!("Hello, world!");
}
```

일단 러스트에서 main 함수는 가장 먼저 실행됨<br>
또한 러스트에서 들여쓰기는 탭이 아닌 공백 4개<br>

println! 는 `러스트 매크로`라고 한다. `!`가 붙어서 함수랑은 다름<br>
또한 세미클론은 꼭 붙여줘야 함

또한, 러스트 프로그램은 `rustc main.rs`과 같이 컴파일러로 먼저 컴파일 해야 함<br>
그 후 컴파일 된 바이너리 파일을 여는 거임

## Cargo 란?

카고는 러스트 빌드 시스템이자 패키지 관리자임<br>
npm과 비슷한 역할을 해준다고 보면 될 듯<br>

### cargo new

새로운 프로젝트를 생성하는 명령어

- src
- Cargo.toml
- .gitignore
  (만약 이미 git이 init 되어 있다면 `cargo new --vcs=git`을 써야 함)
  파일이 생성된다

> cargo new에는 `--bin`, `--lib` 두 가지 flag가 있는데 (`--bin`이 default)
> 다른 프로젝트에서 library 용으로 사용할 프로젝트를 만든다면 `--lib`로
> binary 실행 파일을 만들어야 한다면 `--bin`을 사용한다.
>
> flag에 따라 보일러 플레이트가 다르다.

### Cargo.toml

카고의 설정 파일 형식임<br>
중요한 것은 가장 밑에 적히는 [dependencies] 인데 의존 라이브러리 목록임<br>
러스트에서 패키지는 크레이트(crate)라고 부름

### cargo build

src에 작성한 코드를 빌드할 수 있음

- Cargo.lock
- target 폴더
  가 생기는데 Cargo.lock은 yarn.lock이랑 비슷한 느낌이고
  target 폴더는 빌드된 파일이 모여있음

단, build 변경 사항이 있을 때에만 새로 빌드한다.

### cargo run, check

- run 컴파일된 파일 실행
- check 코드를 컴파일만 하고 실행 파일은 생성하지 않음

웬만해선 컴파일만 하고 싶을 땐 수시로 check를 통해 확인해주자

### cargo build --release

최종적으로 프로젝트를 릴리즈하고 싶다면, 위의 명령어로 최적화된 실행 파일을 생성

---
