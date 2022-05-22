러스트에는 두 가지 error가 있다.<br>
바로 회복이 가능한 에러와 회복이 불가능한 에러

# Unrecoverable Error

예를 들어, `Vec`에 존재하지 않는 index에 접근하려고 한다던지 하면 <br>
runtime 단계에서 panic 에러가 뜨게 된다.

그럼 rust는 그 즉시 프로그램 실행을 중단하고 콘솔에 에러 관련된 사항을 출력하게 된다.

이건 뭐 다른 언어랑 비슷하니까 대충 넘어가고

# Recoverable Error

Error가 발생할 때 실행이 중단되기 보다는,<br>
Error에 처리하고 실행은 계속 진행되기를 바랄 수 있다.<br>
이럴 때 사용하는 것이 바로 `Result` enum이다.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

예를 들어, 다음 코드는 `hello.txt`를 열어 이를 `f` 변수에 저장하는 코드이다.<br>
비록 `hello.txt`가 존재하지 않아도 이 코드는 실행이 중단되지 않는다.<br>
그 이유는 바로 `File::open` 연관함수가 `Result` enum을 반환하고<br>
이 `Result`로 에러를 핸들링을 할 수 있게끔 하기 때문이다.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

> 그럼 특정 함수가 `Result`를 반환해 에러를 처리 할 수 있게끔 제공해주는지<br>
> 우리는 매번 확인해봐야 하는걸까?
>
> 사실 이 때 사용할 수 있는 팁이, `return`값을 저장하는 변수에 타입을 지정해보는 것이다.
>
> ```rust
> let f: u32 = File::open("hello.txt");
> ```
>
> 이와 같이 `f`에 타입을 지정해보면, 컴파일 단계에서 `File::open`은 `Result`를 반환하기에<br> >`f`에 할당할 수 없다고 알려줄 것이다.
>
> 뭐 아니면 확장 프로그램 같은 걸로 알면 되겠지

이렇게 반환된 `Result`는, `Option`처럼 `match` 키워드로 처리해주면 되겠지?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

다만 위와 같이, `Result`를 많이 다루는 함수 같은 경우에는 match를 하도 많이 써야되다보니까<br>
이를 보일러 플레이트로 처리해줄 수 있는 러스트의 특별한 operator가 있는데, 바로 `?`이다.

위에서 match로 처리하던 것을, `Result`를 반환하는 함수 끝에 `?`를 붙여줘서<br>
다음과 같이 간단하게 바꿀 수 있는데

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // open이 Result 리턴
    let mut s = String::new();
    f.read_to_string(&mut s)?; // read_to_string이 Result 리턴
    Ok(s)
}
```

이걸 더 줄이면

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

이렇게도 바꿀 수 있다.
