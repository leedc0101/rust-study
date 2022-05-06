## 일반 프로그래밍 개념

## 변수

`let`, `const` 키워드를 이용하며 두 키워드 모두 기본적으로 불변성을 갖고 있다.<br>
다만 `mut`를 붙이거나 `shadowing`을 이용하면 값을 변경 가능하다.

### let vs const

`let`과 `const`는 둘다 불변성을 갖고 있지만,

1. `const`는 `mut` 키워드를 쓸 수 없고, 선언시 타입을 지정해줘야 한다.
2. `const` 변수는 대문자와 밑줄`_`을 이용하는 것이 국룰이다.
3. 계산된 결과가 아닌, 명시적인 값을 통해 선언해줘야 한다.

```rust
ex)
const MAX_POINTS: u32 = 1000_000;
/// 밑줄은 숫자 가독성 높일 때도 사용할 수 있음 (오?)
```

### mut vs Shadowing

일단 Shadowing이 뭔지부터 보자

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is {}", x); // x is 12
}
```

x 값을 이용해, `let`으로 새로 선언한 x 위에 덮어쓰기를 하고 있는 모습이다.

그럼 `mut`이랑 뭐가 다른걸까?<br>
일단 `let` 키워드로 새로 선언한다는 것도 다르고 무엇보다...

##### 타입에 구애받지 않는다.

```rust
let spaces = "   ";
let spaces = spaces.len(); // 3

let mut spaces = "   ";
spaces = spaces.len(); // compile error!
```

## 데이터 타입

러스트는 정적 타입 언어이다.<br>
즉, 컴파일 시점에 어떤 변수가 어떤 타입을 갖는 지를 알고 있어야 한다.

다음 코드를 봐보자.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

`String`의 `parse` 메소드는 문자를 숫자로 변환시켜준다.<br>
이 때 할당받는 `guess` 변수의 타입을 지정해주지 않으면,<br>
rust에 존재하는 다양한 숫자 타입 중, 무엇을 사용할지 컴파일러 입장에서는 알지 못한다.<br>
따라서, 이런 경우에는 타입 지정을 꼭 해줘야 함.

### 스칼라

러스트에서 다음 네 가지 종류의 스칼라 타입을 정의하고 있다.

- integer
- floating point number
- Bolleans
- charaters

#### 먼저 정수 타입부터 보자

정수 타입은 `u8`, `u16`, `u32`, `u64`와 `i8`, `i16`, `i32`, `i64`가 있다.<br>
저장할 수 있는 범위는 u는 `0 ~ 2^(n)-1`, i는 `-2^(n-1) ~ 2^(n-1)-1`이다.

즉 `u8`은 0~255, `u16`은 0~65535, `u32`는 0~4294967295, <br>
`u64`는 매우 큰 범위까지 커버 가능하다.

러스트는 기본적으로 `i32` 타입을 default로 사용함

[Signed 타입 저장 방식 (Two's complement)](https://en.wikipedia.org/wiki/Two%27s_complement)

##### ![](https://velog.velcdn.com/images/leedc0101/post/5ddc056f-ce49-4cab-889e-f1c792ec9ddb/image.png)

##### ![](https://velog.velcdn.com/images/leedc0101/post/8e7b4100-62c5-4146-bd21-d6a70b1625a9/image.png)

#### 그 다음은 소수점 타입

`f32`와 `f64`가 있는데 러스트는 `f64`를 기본으로 사용함
CPU 연산 성능 상 차이가 거의 없는데, 범위는 훨씬 크기 때문에

#### bolleans는 패스하고..

#### 문자열

##### ![](https://velog.velcdn.com/images/leedc0101/post/f1c83597-4600-4a17-a020-cb59fbc998c1/image.png)

Rust에 워낙 문자와 관련된 타입이 많은데<br>
그 중 가장 원시 타입은 `char`이다.

아스키코드가 아닌 유니코드를 사용하기에<br>
영어는 물론, 한국어와 일본어, 심지어는 이모티콘까지도 저장 가능하다.

자세한 차이점은 [char 공식 문서](https://doc.rust-lang.org/std/primitive.char.html)와 [String 공식 문서](https://doc.rust-lang.org/std/string/struct.String.html) 비교해보면 될 듯

### 컴파운드

컴파운드 타입은 여러 개 값의 그륩 타입이라고 보면 된다. <br>
기본적으로 러스트에서 지원하는 컴파운드 타입은 튜플과 배열이다.

#### 튜플

튜플은 단순히 값의 모임이라고 보면 될 것 같다.<br>
또한 뒤에 나올 배열과 함께 선언과 동시 길이가 고정이다.

타입이 다른 변수들끼리도 묶는게 가능하다.<br>
또한 자바스크립트 객체처럼 `<tupple>.<index>`으로 변수 하나의 값에 접근 가능하고<br>
구조 분해도 가능하다.

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", tup.0);
	println!("The value of y is: {}", y);
}
```

#### 배열

기본적으로 다른 언어와 비슷하지만, 길이가 정해져있음<br>
삭제하거나 추가할 것이 없다면 배열보다는 벡터를 쓰는 것이 좋다. <br>
(이는 추후에 또 알아보자)

```rust
fn main() {
    let a: [i32; 3] = [1,2,3]; // 타입 지정은 이렇게
    let b = [3;5]; // [3,3,3,3,3]
}
```

참고로, 배열에서 유효하지 않은 요소에 접근하면 컴파일은 가능하지만,<br>
Rust의 강점 중 하나인 메모리 관리에 의해 실행 시에 에러가 발생한다

## 함수

함수명은 스네이크 케이스 ( ex. process_mint)<br>
함수 매개변수는 타입을 꼭 명시해줘야 한다.

### 중요! 구문 vs 표현식

구문은 값의 리턴이 없고, 표현식은 최종 결과값으로 리턴이 있다.<br>
함수의 리턴 값은 이름을 부여하진 않지만, 타입은 꼭 정해줘야 됨

```rust
fn main() {
    let y = 6; // 구문
    let x = (let y = 6); // let y = 6는 구문이라 return이 없어서 error
    let x = 5;

    // 코드 블록 또한 표현식이다. 즉 리턴값이 있다는건데
    // 바로 x + 1 와 같이 세미클론이 붙지 않은 저 친구가 표현식임
    let y = {
        let x = 3;
        x + 1 // 세미클론 없는 것에 주목
    }
}
```

물론 return을 붙이면 다른 언어와 비슷하게 작성되나<br>
러스트는 그냥 함수 마지막의 표현식 값을 함수의 리턴값으로 써도 무방하다.

즉, 아래의 두 함수 모두 정상적으로 작동한다.

```rust
fn five() -> u32 {
    return 5;
}

fn six() -> u32 {
    6
}
```

## 흐름 제어

### if문

여타 다른 언어와 동일

다만 Javascript와 같이 0, null 같은 애들을 조건에 넣을 수 없고
무조건 `bool` 타입만

다만 표현식을 이용해 다음과 같은 변수 초기화가 가능

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

뭔가 삼항연산자 같은 느낌<br>
대신 표현식이 return하는 타입이 서로 같아야 한다.

### 반복문

러스트는 loop, while, for 이 세 가지 종류의 반복문을 제공한다.

loop은 기본적으로 조건없이 무한 반복을 실행하는데<br>
break와 함께 return할 표현식을 작성하는 방법으로 중단할 수 있다.

```rust
fn main() {
    let mut counter = 0;

    let rsult = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 20
        }
    };
}
```

if문과 마찬가지로, 변수 초기화도 가능

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // 20
}
```

또한 `for..in` 구문도 작성 가능함<br>
다음과 같이 이터레이션을 사용한 방법을 많이 사용한다고 하니 그거 이용하자.

```rust
fn main() {
    // 1
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // 2
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

```

---
