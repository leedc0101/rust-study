# 첫 프로젝트

## 첫 코드 분석

```rust
use std::io;

fn main () {
    println!("숫자를 맞혀봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요. ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
```

입출력을 위해 `io(input/output)` 라이브러리 가져온다<br>
`io` 라이브러리는 `std`라는 표준 라이브러리에 포함되어 있음

`let mut guess = String::new()`를 보자<br>
일단 기본적으로 러스트 변수는 immutable임<br>
하지만 변경할 수 있는 변수를 생성하기 위해 `mut`키워드를 사용중

`String::new()` 는 String 타입의 새 인스턴스를 만드는 것<br>
그 인스턴스를 guess 변수에 바인딩했다고 보면 됨 (`new`는 String의 메소드)

<br>

`io::stdin().read_line(&mut guess).expect("입력~~.")`

이제 `io`의 메소드인 `stdin`을 이용해 사용자의 입력값을 읽을 수 있다.<br>
여기서 `&mut guess`는 참조인데, 데이터를 복사할 필요 없이 접근하게 해주는 기능<br>
포인터 같은 개념 같은데, 4장에서 다시 다룬다고 하니 그때 다시 공부하자

`expect`는 에러 처리 메서드인데 안 쓰면 경고가 뜸<br>
(에러를 꼼꼼히 다 처리하라는 러스트의 배려.. ㄷㄷ)

<br>

## 난수 생성

러스트에서 난수 생성 기능을 표준으로 제공하진 않는다.<br>
다만 rand라는 라이브러리(앞으로는 크레이트라고 하자)을 제공함<br>
크레이트를 사용하기 전에 Cargo.toml에서 dependencies를 수정해주자

> 난 약간 yarn add 에 익숙해져 있다보니, 뭔가 손으로 dependencies 쓰는게 귀찮아서
> [이 링크](https://stackoverflow.com/questions/32934293/is-there-a-command-to-automatically-add-a-crate-to-my-cargo-toml)를 참고해서 cargo install 이용했다.

##### 다만...

특정 라이브러리 버전이 필요할 때는 버전을 적어줘야 한다.<br>
버전 적는 방식과 호환성은 솔리디티와 비슷한 듯
[자세한 사용법은 여기에](https://crates.io/crates/cargo-edit)

##### 문제 발생!

책에서는 rand 크레이트 0.6.1 버전을 사용하는데 cargo install로 설치하니<br>
가장 최신 버전의 0.8.4가 설치되었다.<br>
그래서 메소드의 인자 갯수가 달라 에러가 났는데, 이럴땐 라이브러리 보고 고쳐야 함

### cargo doc --open

이럴땐 물론 구글링해서 찾아도 되지만, 터미널에서 위의 명령어를 치면<br>
현재 프로젝트에서 쓰고 있는 라이브러리의 문서를 정리해서 웹페이지로 알아서 띄워준다.<br>
구글링도 좋지만, 이 방법이 묶어서 한번에 보여주니 편하긴 한듯

예를 들어 `rand::thread_rng().gen_range` 메소드가 변경되어서 에러가 났었는데<br>
이럴 때는 아래와 같이 검색창에 메소드를 직접 검색해서 사용법을 확인하면 됨<br>

###### ![](https://velog.velcdn.com/images/leedc0101/post/939d4ea3-6af6-4cae-b84f-9e10ba0d360c/image.png)

<br>

### 최종코드

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut count = 0;

    loop {
        count += 1;
        println!("숫자를 맞춰보세요! ({}번째 시도)", count);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("숫자를 입력하세요!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다! \n"),
            Ordering::Greater => println!("입력한 숫자가 큽니다! \n"),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
        }
    }
}
```

책의 코드에서 몇 가지 feature를 좀 추가했다
