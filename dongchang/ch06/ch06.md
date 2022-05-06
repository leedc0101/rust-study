## 열거자 (Enum)
열거자란 형식의 개념은 비슷하나 서로 다른 구조를 가지고 있는 타입을 생성할 때 사용한다.

예를 들면 IP 주소가 있다. <br>
현재 IP 주소는 v4, v6이 있는데, 서로 개념은 비슷하나 데이터 구조는 서로 다르다.<br>
구조체로는 이를 구현하기 어렵지만, 열거자를 사용하면 구현 가능하다.

먼저 구조체로 이를 구현하면
```rust 
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}
```

구조체를 이용해서 enum 타입인 kind와 String 타입인 address를 묶었다<br>
하지만, 구조체를 사용하지 않고도 열거자만 이용해도, 데이터를 열것값에 직접 넣을 수 있다.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.01"));

let loopback = IpAddr::V6(String::from("::1"));
```
위에서 구조체로 묶었던 것을, enum안에 직접 표현해준 모습을 볼 수 있다.<br>
또한 열것값마다 데이터 구조를 다양한 방법으로 정의할 수 있다는 것도 장점이다.

최종적으로 가장 보편적으로 `Enum`을 정의하는 방법은 다음과 같다.
```rust
struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
보다시피 String, u8 뿐만 아니라 구조체, 심지어 열거자를 저장해도 무방하다.<br>
물론 Quit처럼 연관 데이터를 전혀 갖지 않아도 된다.<br>
중요한 점은 Quit, Move, Write, ChangeColor 모두 Message 타입에 속한다는 것이다.

> 이때 Quit과 같이 필드를 갖지 않는 구조체를 `Unit Struct`라고 하는데, <br>
말그대로 형식만을 나타내주는 구조체이다. <br>
후에 match 함수에서 조건에 따라 처리를 다르게 해줄 때 있어서 유용하다.

또한, 열거자에 메소드를 정의할 수 있다.
```rust
impl Message {
    fn call(&self) {}
}55

let m = Message::Write(String::from("hello"));

m.call();
```
이런 식으로

<br>

## Option과 Null
통상적으로 프로그래밍 언어에서 비어있는 변수는 0, ""가 아닌 null로 표현할 때가 많다.<br>
(자바스크립트에서는 undefined이나 NaN도 있고)

근데 이런 null을 null이 아닌 값처럼 사용하려고 하면 당연히 에러가 나는데,<br>
이런 에러가 너무나도 많이, 그리고 쉽게 발생한다는 것이 문제 중 하나이다.
```rust 
enum Option<T> {
   Some(T),
   None,
}
```
러스트의 표준 라이브러리가 제공하는 열거자이다. <br>
`T`는 구조체를 포함한 어떤 타입이든 들어갈 수 있다는 뜻이다.

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
이 코드를 실행하면 에러가 난다.<br>
`i8`과 `Option<i8>`은 타입이 다르기 때문이다.

이렇게 구분해두면, null값이 필요한 상황에서는 Option 열거자를 사용하고,<br>
null 값이 필요없을 때는 그냥 자료형을 써주면 된다.

<br>

## match

Enum을 이용해 조건문을 작성할 때 아주 유용한 키워드다. <br>
또한, Enum 안에 들어있는 값을 바인딩해서 사용할 수도 있다.


```rust
enum UsState {
    Alabama, Alaska, ....
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    value_in_cents(
        Coin::Quater(UsState::Alaska)
    ); // State quarter from Alaska!
```

또한 `Option<T>`과 같이 사용하기도 좋다 <br>
아래의 코드를 보면, `plus_one`함수는 `i32`가 아닌 `Option<i32>`를 인자로 받는데,<br>
이 때 인자가 `None` 타입이면 (1)과 같이 `None`을 반환하고<br>
`Some`이라면 (2)와 같이 Enum안에 있는 값을 `i`에 바인딩해서, `Some(i+1)`을 반환한다.

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // (1)
            Some(i) => Some(i + 1), // (2)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

<br>

## 예외 케이스
match는 기본적으로 모든 Case를 다 다루어야한다. <br>
만약 그렇지 않다면, 컴파일 에러를 반환하게 됨

만약, 몇개의 case 이외에 나머지 case들을 같은 로직을 처리 해야 한다면
가장 마지막에 이를 선언해주면 된다.

이 때 2가지 경우가 있을 수 있는데, enum 안의 값을 바인딩 할 필요가 있을 때와 없을 때이다.

```rust
// 바인딩 할 때
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // other에 값 바인딩 함
}

// 바인딩 필요 없을 때
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
```

<br>

### 그런데..
굳이 바인딩이 필요 없을 때 처리할 로직이 없다면 match를 써야할까..? <br>
에를 들어 다음의 코드를 봐보자
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```
max에 어떠한 값도 없다면 아무것도 처리하지 않는 match 함수가 있다. <br>
딱 봐도 `- => ()` 라인이 별로 쓸모가 없어보인다.<br>
변수가 Some일 때만 어떠한 로직을 처리하도록 바꿀 수 있는 방법이 없을까?

이 때 유용한 syntax가

## `if` `let`
위의 코드를 `if` `let`을 이용하면 다음과 같이 바꿀 수 있다.
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("The maximum is configured to be {}", max);
}
```

한결 코드가 줄어들었다.

심지어 `else`를 넣어서 예외 케이스 로직을 처리할 수도 있다.<br>
다만, `match`처럼 바인딩이 필요한 경우는 처리할 수 없으니<br>
enum 안의 값을 사용해야 한다면 `match`를 이용하도록 하자.

