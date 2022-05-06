# 구조체
마치 자바스크립트 객체와 비슷하다.<br>
다만, 구조체의 몇몇 필드만을 가변 데이터로 표시할 수 없고,<br>
구조체 인스턴스 자체가 가변이거나 불가변이거나 둘 중 하나

##### 주의점
> 구조체 타입은 구조체가 소유권을 갖는 타입으로 지정해주거나 
참조 타입이라면 `lifetimes`를 지정해주어야 한다.<br>
다만 `lifetimes`은 나중에 다룰 개념이기에, 일단은 `&str`보다는 `String`을 쓰도록 하자

<br>

## derive 에노테이션
```rust
struct Rectangle {
    width: u32, 
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("사각형의 면적 : {} 제곱 픽셀", rect1.width)); // 30
    println!("rect1: {}", rect1); // error
}
```
위에서 `rect1.width`과 같이 단일 값은 프린트할 때 형식을 지정할 필요가 없지만<br>
`rect1`와 같이 구조체들은 프린트할 때 형식을 지정해줘야 한다.

물론 자바스크립트에서 `console.log`하면 알아서 보여주듯이, <br>
러스트도 그런 기능이 있는데 바로 `#[derive(Debug)]`임

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("사각형의 면적 : {} 제곱 픽셀", rect1.width)); // 30
    println!("{:?}", rect1); // Rectangle { width: 30, height: 50 }
}
```
이렇게 하면 `console.log`와 같이 구조체의 모든 필드값을 확인 가능해진다.

<br>

## 메소드
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
	let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    
    println!("rect1는 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
}
```

다음과 같이 메소드를 직접 정의할 수 있다.

`&self`는 자기 자신을 참조한다는 의미인데, 그렇기에 타입을 적을 필요가 없다.<br>
다른 인자를 받고 싶을 때는, `other: &Rectangle`과 같이 적어주면 된다.

<br>

## 연관함수
``String::from``과 같이 self 매개변수를 사용하지 않는 함수를 연관 함수라고 한다.<br>
생성자 함수 같은 애들이 연관 함수의 예이다 (ex `String::from("Hello")`)

연관 함수 호출하려면 `::` 문법 사용하면 됨

자바스크립트로 따지면 Static Method와 비슷한듯?

<br>

## 구조 분해 할당
자바스크립트 `...obj`처럼 러스트도 `..obj`를 지원한다. (점 2개인 것에 유의)

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```